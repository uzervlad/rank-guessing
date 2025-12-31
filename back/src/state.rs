use std::{
	env,
	sync::{
		Arc,
		Mutex,
		atomic::{AtomicI64, AtomicUsize, Ordering},
	},
};

use axum::body::Bytes;
use chrono::{DateTime, TimeDelta, Utc};
use eyre::Result;
use rosu_v2::Osu;
use serde::Serialize;
use sqlx::SqlitePool;
use tokio::sync::{broadcast::{self, Receiver, Sender}};

use crate::{database::{self, requests::RequestsCount}, emotes::fetch_emotes};

pub struct OsuConfig {
	pub client_id: u64,
	pub client_secret: String,
	pub redirect_uri: String,
}

impl OsuConfig {
	fn new() -> Result<Self> {
		Ok(Self {
			client_id: env::var("OSU_CLIENT_ID")?.parse()?,
			client_secret: env::var("OSU_CLIENT_SECRET")?,
			redirect_uri: env::var("OSU_REDIRECT_URI")?,
		})
	}

	pub fn to_auth_url(&self) -> String {
		format!(
			"https://osu.ppy.sh/oauth/authorize?client_id={}&client_secret={}&response_type=code&scope=identify&redirect_uri={}",
			self.client_id, self.client_secret, self.redirect_uri
		)
	}

	async fn create_client(&self) -> Result<Osu> {
		let client = Osu::builder()
			.client_id(self.client_id)
			.client_secret(&self.client_secret)
			.build()
			.await?;

		Ok(client)
	}
}

pub struct AppConfig {
	pub osu: OsuConfig,
	pub jwt_secret: String,
	pub frontend_url: String,
	pub guesser_id: u64,
	pub admin_id: u64,
	pub twitch_client_id: String,
	pub twitch_client_secret: String,
}

impl AppConfig {
	fn new() -> Result<Self> {
		Ok(Self {
			osu: OsuConfig::new()?,
			jwt_secret: env::var("JWT_SECRET")?,
			frontend_url: env::var("FRONTEND_URL")?,
			guesser_id: env::var("GUESSER_ID")?.parse()?,
			admin_id: env::var("ADMIN_ID")?.parse()?,
			twitch_client_id: env::var("TWITCH_CLIENT_ID")?,
			twitch_client_secret: env::var("TWITCH_CLIENT_SECRET")?,
		})
	}
}

#[derive(Serialize)]
pub struct AppStateBroadcast {
	total_submissions: usize,
	ready_submissions: usize,
	session_id: i64,
}

impl From<Arc<AppState>> for AppStateBroadcast {
	fn from(state: Arc<AppState>) -> Self {
		Self {
			total_submissions: state.total_submissions.load(Ordering::SeqCst),
			ready_submissions: state.ready_submissions.load(Ordering::SeqCst),
			session_id: state.session_id.load(Ordering::SeqCst),
		}
	}
}

pub struct AppState {
	pub total_submissions: AtomicUsize,
	pub ready_submissions: AtomicUsize,
	pub session_id: AtomicI64,

	tx: Sender<Arc<AppStateBroadcast>>,
}

impl AppState {
	fn new(tx: Sender<Arc<AppStateBroadcast>>, count: RequestsCount, session_id: i64) -> Self {
		Self {
			total_submissions: (count.total as usize).into(),
			ready_submissions: (count.ready as usize).into(),
			session_id: session_id.into(),

			tx,
		}
	}

	pub fn subscribe(&self) -> Receiver<Arc<AppStateBroadcast>> {
		self.tx.subscribe()
	}
}

pub trait ArcAppStateTrait {
	fn broadcast(&self);

	fn total_add(&self);
	fn total_sub(&self);
	fn ready_add(&self);
	fn ready_sub(&self);
	fn set_session_id(&self, id: i64);
	fn reset(&self);
}

impl ArcAppStateTrait for Arc<AppState> {
	fn broadcast(&self) {
		let _self = self.clone();

		tokio::spawn(async move {
			let message = AppStateBroadcast::from(_self.clone());

			let _ = _self.tx.send(Arc::new(message));
		});
	}

	fn total_add(&self) {
		self.total_submissions.fetch_add(1, Ordering::SeqCst);
		self.broadcast();
	}

	fn total_sub(&self) {
		self.total_submissions.fetch_sub(1, Ordering::SeqCst);
		self.broadcast();
	}

	fn ready_add(&self) {
		self.ready_submissions.fetch_add(1, Ordering::SeqCst);
		self.broadcast();
	}

	fn ready_sub(&self) {
		self.ready_submissions.fetch_sub(1, Ordering::SeqCst);
		self.broadcast();
	}

	fn set_session_id(&self, id: i64) {
		self.session_id.store(id, Ordering::SeqCst);
	}

	fn reset(&self) {
		self.ready_submissions.store(0, Ordering::SeqCst);
		self.total_submissions.store(0, Ordering::SeqCst);
		self.session_id.store(0, Ordering::SeqCst);
	}
}

pub struct TwitchState {
	pub vod_id: String,
	pub started_at: DateTime<Utc>,
}

fn format_vod_timestamp(timestamp: TimeDelta) -> String {
	let mut seconds = timestamp.num_seconds();
	let hours = seconds / 3600;
	seconds %= 3600;
	let minutes = seconds / 60;
	seconds %= 60;

	format!("{hours:02}h{minutes:02}m{seconds:02}s")
}

impl TwitchState {
	pub fn get_link_at(&self, at: DateTime<Utc>) -> String {
		let timestamp = format_vod_timestamp(at - self.started_at);
		format!("https://www.twitch.tv/videos/{}?t={}", self.vod_id, timestamp)
	}
}

pub struct AxumState {
	pub config: Arc<AppConfig>,
	pub state: Arc<AppState>,
	pub emotes: Bytes,
	pub twitch: Arc<Mutex<Option<TwitchState>>>,
	pub db: SqlitePool,
	pub osu: Osu,
}

impl AxumState {
	pub async fn new(db: SqlitePool) -> Result<Self> {
		let (tx, _) = broadcast::channel(16);

		let config = Arc::new(AppConfig::new()?);

		let state = {
			let session_id = database::sessions::get_current_session(&db)
				.await?
				.map(|s| s.id)
				.unwrap_or(0);
			let count = match session_id {
				0 => Default::default(),
				id => database::requests::count_requests(&db, id).await?,
			};

			Arc::new(AppState::new(tx, count, session_id))
		};

		let osu = config.osu.create_client().await?;

		let emotes = fetch_emotes().await;
		let emotes = serde_json::to_vec(&emotes)?;
		let emotes = Bytes::from(emotes);

		Ok(Self {
			config,
			state,
			emotes,
			twitch: Arc::new(Mutex::new(None)),
			db,
			osu,
		})
	}
}

pub type AAxumState = Arc<AxumState>;
