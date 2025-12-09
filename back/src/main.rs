#![allow(unused)]

use std::{env, fs, sync::Arc, time::Instant};

use axum::{Json, Router, extract::State, routing::get};
use back::{
	replay::Replay,
	routes,
	state::{AAxumState, AxumState},
};
use eyre::Result;
use serde::Serialize;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
	let _ = dotenvy::dotenv();

	let db = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

	#[cfg(not(debug_assertions))]
	sqlx::migrate!("./migrations").run(&db).await?;

	let state = AxumState::new(db.clone()).await?;
	let state = Arc::new(state);

	let app = Router::new()
		.route("/", get(root))
		.nest("/auth", routes::auth::router(state.clone()))
		.nest("/play", routes::play::router(state.clone()))
		.nest("/session", routes::session::router(state.clone()))
		.nest("/request", routes::request::router(state.clone()))
		.nest("/state", routes::state::router())
		.nest("/emotes", routes::emotes::router())
		.with_state(state);

	let listener = TcpListener::bind("0.0.0.0:3999").await?;

	axum::serve(listener, app).await?;

	Ok(())
}

async fn root() -> &'static str {
	"Hello, World!"
}
