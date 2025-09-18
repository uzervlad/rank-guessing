use chrono::{DateTime, Utc};
use eyre::Result;
use serde::Serialize;
use sqlx::{SqlitePool, prelude::FromRow};

use crate::replay::analysis::{ClientState, OnlineState, UserState};

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
	pub guessed_rank: i64,
	pub real_rank: i64,
}

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

pub async fn get_requests_by_session(pool: &SqlitePool, session_id: i64) -> Result<Vec<DbRequest>> {
	let request = sqlx::query_as::<_, DbRequest>(
		r#"
    select * from requests
    where session_id = $1
  "#,
	)
	.bind(session_id)
	.fetch_all(pool)
	.await?;

	Ok(request)
}

pub async fn get_requests_by_player(pool: &SqlitePool, player_id: u64) -> Result<Vec<DbRequest>> {
	let request = sqlx::query_as::<_, DbRequest>(
		r#"
    select * from requests
    where player_id = $1
  "#,
	)
	.bind(player_id as i64)
	.fetch_all(pool)
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
) -> Result<i64> {
	let id = sqlx::query_scalar::<_, i64>(
		r#"
    insert into requests
      (player_id, beatmap_id, session_id, client_state, user_state, online_state, submitted_at)
      values ($1, $2, $3, $4, $5, $6, $7)
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

pub async fn mark_as_guessed_request(
	pool: &SqlitePool,
	guessed_rank: u32,
	real_rank: u32,
	request_id: i64
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

pub async fn delete_all_requests(pool: &SqlitePool) -> Result<()> {
	sqlx::query("delete from requests").execute(pool).await?;

	Ok(())
}

#[derive(FromRow, Default)]
pub struct RequestsCount {
	pub total: i64,
	pub ready: i64,
}

// select count(*) as total, coalesce(sum(case when ready and watched_at is null then 1 else 0 end), 0) from requests

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
