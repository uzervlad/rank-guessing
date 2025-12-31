use std::{sync::{Arc, Mutex}, time::Duration};

use chrono::{DateTime, Utc};
use eyre::Result;
use serde::Deserialize;

use crate::state::{AppConfig, TwitchState};

#[derive(Deserialize)]
struct TokenResponse {
	access_token: String,
	expires_in: u64,
}

struct TwitchClient {
	client: reqwest::Client,
	config: Arc<AppConfig>,
	access_token: String,
	expires_at: DateTime<Utc>,
}

impl TwitchClient {
	async fn new(config: Arc<AppConfig>) -> Result<Self> {
		let client = reqwest::Client::builder()
			.user_agent("rank-guessing v2")
			.build()?;

		let response = client
			.post("https://id.twitch.tv/oauth2/token")
			.query(&[
				("client_id", config.twitch_client_id.as_str()),
				("client_secret", config.twitch_client_secret.as_str()),
				("grant_type", "client_credentials"),
			])
			.send()
			.await?
			.error_for_status()?
			.json::<TokenResponse>()
			.await?;

		Ok(Self {
			client,
			config,
			access_token: response.access_token,
			expires_at: Utc::now() + chrono::Duration::seconds((response.expires_in - 120) as _),
		})
	}

	async fn refresh(&mut self) -> Result<()> {
		let response = self.client
			.post("https://id.twitch.tv/oauth2/token")
			.query(&[
				("client_id", self.config.twitch_client_id.as_str()),
				("client_secret", self.config.twitch_client_secret.as_str()),
				("grant_type", "client_credentials"),
			])
			.send()
			.await?
			.error_for_status()?
			.json::<TokenResponse>()
			.await?;

		self.access_token = response.access_token;
		self.expires_at = Utc::now() + chrono::Duration::seconds((response.expires_in - 120) as _);

		Ok(())
	}
	
	fn token_expired(&self) -> bool {
		Utc::now() > self.expires_at
	}

	async fn get_streams(&mut self, user: &str) -> Result<StreamsResponse> {
		if self.token_expired() {
			self.refresh().await?;
		}

		let streams = self.client
			.get("https://api.twitch.tv/helix/streams")
			.query(&[("user_login", user)])
			.header("Client-ID", &self.config.twitch_client_id)
			.bearer_auth(&self.access_token)
			.send()
			.await?
			.error_for_status()?
			.json::<StreamsResponse>()
			.await?;

		Ok(streams)
	}
}

#[derive(Debug, Deserialize)]
struct StreamsResponse {
	data: Vec<Stream>,
}

#[derive(Debug, Deserialize)]
struct Stream {
	id: String,
	started_at: DateTime<Utc>,
}

pub async fn twitch_thread(
	config: Arc<AppConfig>,
	state: Arc<Mutex<Option<TwitchState>>>
) -> Result<()> {
	let mut client = TwitchClient::new(config).await?;

	loop {
		match client.get_streams("koifishu").await {
			Ok(streams) => {
				match streams.data.first() {
					Some(stream) => *state.lock().unwrap() = Some(TwitchState {
						vod_id: stream.id.clone(),
						started_at: stream.started_at.clone(),
					}),
					None => *state.lock().unwrap() = None,
				}
			},
			_ => {},
		}

		tokio::time::sleep(Duration::from_secs(30)).await;
	}
}