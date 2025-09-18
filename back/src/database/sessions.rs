use chrono::{DateTime, Utc};
use eyre::Result;
use serde::Serialize;
use sqlx::{SqlitePool, prelude::FromRow};

#[derive(FromRow, Serialize)]
pub struct DbSession {
	pub id: i64,
	pub title: String,
	pub started_at: DateTime<Utc>,
	pub ended_at: Option<DateTime<Utc>>,
}

#[derive(FromRow, Serialize)]
pub struct DbSessionExtended {
	pub id: i64,
	pub title: String,
	pub started_at: DateTime<Utc>,
	pub ended_at: Option<DateTime<Utc>>,
	pub submissions: i64,
	pub guesses: i64,
}

pub async fn get_sessions(pool: &SqlitePool) -> Result<Vec<DbSessionExtended>> {
	let sessions = sqlx::query_as::<_, DbSessionExtended>(
		r#"
    select
			sessions.*,
			coalesce(count(requests.id), 0) as submissions,
			coalesce(count(case when requests.watched_at is not null then 1 end), 0) as guesses
		from sessions
		left join requests
			on requests.session_id = sessions.id
		group by sessions.id
  "#,
	)
	.fetch_all(pool)
	.await?;

	Ok(sessions)
}

pub async fn get_current_session(pool: &SqlitePool) -> Result<Option<DbSession>> {
	let session = sqlx::query_as::<_, DbSession>(
		r#"
    select * from sessions
    where ended_at is null
  "#,
	)
	.fetch_optional(pool)
	.await?;

	Ok(session)
}

pub async fn create_session(pool: &SqlitePool, title: String) -> Result<i64> {
	let id = sqlx::query_scalar::<_, i64>(
		r#"
    insert into sessions
    (title, started_at) values ($1, $2)
		returning id
  "#,
	)
	.bind(&title)
	.bind(Utc::now())
	.fetch_one(pool)
	.await?;

	Ok(id)
}

pub async fn end_session(pool: &SqlitePool) -> Result<()> {
	sqlx::query(
		r#"
    update sessions
    set ended_at = $1
    where ended_at is null
  "#,
	)
	.bind(Utc::now())
	.execute(pool)
	.await?;

	Ok(())
}

pub async fn rename_session(pool: &SqlitePool, name: String) -> Result<()> {
	sqlx::query(r#"
		update sessions
		set title = $1
		where ended_at is null
	"#)
	.bind(name)
	.execute(pool)
	.await?;

	Ok(())
}
