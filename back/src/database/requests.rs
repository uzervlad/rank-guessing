use chrono::{DateTime, Utc};
use eyre::Result;
use serde::Serialize;
use sqlx::{SqlitePool, prelude::FromRow};

use crate::{
	database::beatmaps::DbBeatmap,
	replay::analysis::{ClientState, OnlineState, UserState},
};

#[derive(FromRow, Serialize)]
pub struct DbRequest {
	pub id: i64,
	pub player_id: i64,
	pub session_id: i64,
	pub beatmap_id: i64,
	pub client_state: ClientState,
	pub online_state: OnlineState,
	pub user_state: UserState,
	pub ready: bool,
	pub submitted_at: DateTime<Utc>,
	pub watched_at: Option<DateTime<Utc>>,
	pub vod_link: Option<String>,
	pub guessed_rank: i64,
	pub real_rank: i64,
	pub comment: String,
	pub additional_notes: Option<String>,
}

#[derive(FromRow)]
pub struct DbRequestWithBeatmap {
	pub r_id: i64,
	pub r_player_id: i64,
	pub r_session_id: i64,
	pub r_beatmap_id: i64,
	pub r_client_state: ClientState,
	pub r_online_state: OnlineState,
	pub r_user_state: UserState,
	pub r_ready: bool,
	pub r_submitted_at: DateTime<Utc>,
	pub r_watched_at: Option<DateTime<Utc>>,
	pub r_vod_link: Option<String>,
	pub r_guessed_rank: i64,
	pub r_real_rank: i64,
	pub r_comment: String,
	pub r_additional_notes: Option<String>,

	pub b_id: i64,
	pub b_beatmapset_id: i64,
	pub b_title: String,
	pub b_artist: String,
	pub b_version: String,
	pub b_creator: String,
}

#[derive(Serialize)]
pub struct RequestWithBeatmap {
	request: DbRequest,
	beatmap: DbBeatmap,
}

impl From<DbRequestWithBeatmap> for RequestWithBeatmap {
	fn from(value: DbRequestWithBeatmap) -> Self {
		Self {
			request: DbRequest {
				id: value.r_id,
				player_id: value.r_player_id,
				session_id: value.r_session_id,
				beatmap_id: value.r_beatmap_id,
				client_state: value.r_client_state,
				online_state: value.r_online_state,
				user_state: value.r_user_state,
				ready: value.r_ready,
				submitted_at: value.r_submitted_at,
				watched_at: value.r_watched_at,
				vod_link: value.r_vod_link,
				guessed_rank: value.r_guessed_rank,
				real_rank: value.r_real_rank,
				comment: value.r_comment,
				additional_notes: value.r_additional_notes,
			},
			beatmap: DbBeatmap {
				id: value.b_id,
				beatmapset_id: value.b_beatmapset_id,
				title: value.b_title.clone(),
				artist: value.b_artist.clone(),
				version: value.b_version.clone(),
				creator: value.b_creator.clone(),
			},
		}
	}
}

const EXTENDED_SELECT: &'static str = r#"
	r.id as r_id,
	r.player_id as r_player_id,
	r.session_id as r_session_id,
	r.beatmap_id as r_beatmap_id,
	r.client_state as r_client_state,
	r.online_state as r_online_state,
	r.user_state as r_user_state,
	r.ready as r_ready,
	r.submitted_at as r_submitted_at,
	r.watched_at as r_watched_at,
	r.vod_link as r_vod_link,
	r.guessed_rank as r_guessed_rank,
	r.real_rank as r_real_rank,
	r.comment as r_comment,
	r.additional_notes as r_additional_notes,

	b.id as b_id,
	b.beatmapset_id as b_beatmapset_id,
	b.title as b_title,
	b.artist as b_artist,
	b.version as b_version,
	b.creator as b_creator
"#;

pub async fn get_request(pool: &SqlitePool, id: i64) -> Result<Option<DbRequest>> {
	let request = sqlx::query_as::<_, DbRequest>(
		r#"
    select * from requests
    where id = $1
  "#,
	)
	.bind(id)
	.fetch_optional(pool)
	.await?;

	Ok(request)
}

pub async fn get_requests_by_session(
	pool: &SqlitePool,
	session_id: i64,
	guessed: bool,
) -> Result<Vec<RequestWithBeatmap>> {
	let mut query = format!(
		r#"
		select {EXTENDED_SELECT} from requests r
		left join beatmaps b on r.beatmap_id = b.id
		where r.session_id = $1
	"#
	);

	if guessed {
		query.push_str("and r.watched_at is not null");
	}

	let request = sqlx::query_as::<_, DbRequestWithBeatmap>(&query)
		.bind(session_id)
		.fetch_all(pool)
		.await?;

	Ok(request
		.into_iter()
		.map(|raw| RequestWithBeatmap::from(raw))
		.collect())
}

pub async fn get_requests_by_player(
	pool: &SqlitePool,
	player_id: u64,
) -> Result<Vec<RequestWithBeatmap>> {
	let query = format!(
		r#"
    select {EXTENDED_SELECT} from requests r
		left join beatmaps b on r.beatmap_id = b.id
    where r.player_id = $1
  "#
	);

	let request = sqlx::query_as::<_, DbRequestWithBeatmap>(&query)
		.bind(player_id as i64)
		.fetch_all(pool)
		.await?;

	Ok(request
		.into_iter()
		.map(|raw| RequestWithBeatmap::from(raw))
		.collect())
}

pub async fn get_last_request_by_player(
	pool: &SqlitePool,
	player_id: u64,
) -> Result<Option<DbRequest>> {
	let request = sqlx::query_as::<_, DbRequest>(
		r#"
		select * from requests
		where player_id = $1
		order by id desc
		limit 1
	"#,
	)
	.bind(player_id as i64)
	.fetch_optional(pool)
	.await?;

	Ok(request)
}

pub async fn get_request_by_session_player(
	pool: &SqlitePool,
	session_id: i64,
	player_id: u64,
) -> Result<Option<DbRequest>> {
	let request = sqlx::query_as::<_, DbRequest>(
		r#"
    select * from requests
    where session_id = $1 and player_id = $2
  "#,
	)
	.bind(session_id)
	.bind(player_id as i64)
	.fetch_optional(pool)
	.await?;

	Ok(request)
}

pub async fn random_request(pool: &SqlitePool, session_id: i64) -> Result<Option<DbRequest>> {
	let request = sqlx::query_as::<_, DbRequest>(
		r#"
    select * from requests
    where session_id = $1 and ready and watched_at is null
    order by random()
    limit 1
  "#,
	)
	.bind(session_id)
	.fetch_optional(pool)
	.await?;

	Ok(request)
}

pub async fn create_request(
	pool: &SqlitePool,
	player_id: u64,
	beatmap_id: u32,
	session_id: i64,
	client_state: ClientState,
	user_state: UserState,
	online_state: OnlineState,
	comment: Option<String>,
) -> Result<i64> {
	let id = sqlx::query_scalar::<_, i64>(
		r#"
    insert into requests
      (player_id, beatmap_id, session_id, client_state, user_state, online_state, submitted_at, comment)
      values ($1, $2, $3, $4, $5, $6, $7, $8)
    returning id
  "#,
	)
	.bind(player_id as i64)
	.bind(beatmap_id as i32)
	.bind(session_id)
	.bind(client_state)
	.bind(user_state)
	.bind(online_state)
	.bind(Utc::now())
	.bind(comment)
	.fetch_one(pool)
	.await?;

	Ok(id)
}

pub async fn ready_request(pool: &SqlitePool, request_id: i64) -> Result<()> {
	sqlx::query(
		r#"
    update requests
    set ready = true
    where id = $1
  "#,
	)
	.bind(request_id)
	.execute(pool)
	.await?;

	Ok(())
}

pub async fn change_request_comment(pool: &SqlitePool, request_id: i64, comment: String) -> Result<()> {
	sqlx::query(
		r#"
		update requests
		set comment = $1
		where id = $2
	"#
	)
	.bind(comment)
	.bind(request_id)
	.execute(pool)
	.await?;

	Ok(())
}

pub async fn mark_as_guessed_request(
	pool: &SqlitePool,
	request_id: i64,
	guessed_rank: u32,
	real_rank: u32,
) -> Result<()> {
	sqlx::query(
		r#"
    update requests
    set
			watched_at = $2,
			guessed_rank = $3,
			real_rank = $4
    where id = $1
  "#,
	)
	.bind(request_id)
	.bind(Utc::now())
	.bind(guessed_rank)
	.bind(real_rank)
	.execute(pool)
	.await?;

	Ok(())
}

pub async fn attach_vod_to_request(
	pool: &SqlitePool,
	request_id: i64,
	link: &str,
) -> Result<()> {
	sqlx::query(
		r#"
		update requests
		set
			vod_link = $2
		where id = $1
	"#,
	)
	.bind(request_id)
	.bind(link)
	.execute(pool)
	.await?;

	Ok(())
}

pub async fn set_additional_notes(
	pool: &SqlitePool,
	request_id: i64,
	note: Option<String>,
) -> Result<()> {
	sqlx::query(
		r#"
		update requests
			set additional_notes = $2
		where id = $1
		"#,
	)
	.bind(request_id)
	.bind(note)
	.execute(pool)
	.await?;

	Ok(())
}

pub async fn delete_request(pool: &SqlitePool, request_id: i64) -> Result<()> {
	sqlx::query(
		r#"
    delete from requests
    where id = $1
  "#,
	)
	.bind(request_id)
	.execute(pool)
	.await?;

	Ok(())
}

#[derive(FromRow, Default)]
pub struct RequestsCount {
	pub total: i64,
	pub ready: i64,
}

pub async fn count_requests(pool: &SqlitePool, session_id: i64) -> Result<RequestsCount> {
	let count = sqlx::query_as::<_, RequestsCount>(
		r#"
    select
      count(*) as total,
      coalesce(sum(case when ready and watched_at is null then 1 else 0 end), 0) as ready
    from requests
		where session_id = $1
  "#,
	)
	.bind(session_id)
	.fetch_one(pool)
	.await?;

	Ok(count)
}
