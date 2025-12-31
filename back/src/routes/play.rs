use std::sync::atomic::Ordering;

use axum::{
	Json, Router, extract::{Path, Query, Request, State}, http::{HeaderValue, StatusCode, header}, middleware, response::IntoResponse, routing::{get, post}
};
use chrono::{Duration, Utc};
use rosu_v2::model::GameMode;
use serde::{Deserialize, Serialize};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{
	auth,
	database::{self, beatmaps::DbBeatmap, requests::DbRequest},
	state::{AAxumState, ArcAppStateTrait},
};

#[derive(Deserialize)]
struct GetRequestQuery {
	id: Option<i64>,
}

#[derive(Serialize)]
struct GetRequestResponse {
	request: DbRequest,
	beatmap: DbBeatmap,
}

async fn get_request(
	State(state): State<AAxumState>,
	Query(query): Query<GetRequestQuery>,
) -> impl IntoResponse {
	let session_id = state.state.session_id.load(Ordering::SeqCst);

	let Ok(request) = (match query.id {
		Some(id) => database::requests::get_request(&state.db, id).await,
		_ => database::requests::random_request(&state.db, session_id).await,
	}) else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error",
		)
			.into_response();
	};

	let Some(request) = request else {
		return (
			StatusCode::NOT_FOUND,
			match query.id {
				Some(_) => "Request not found",
				_ => "No requests available",
			},
		)
			.into_response();
	};

	let Ok(beatmap) = database::beatmaps::get_beatmap(&state.db, request.beatmap_id).await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error",
		)
			.into_response();
	};

	let Some(beatmap) = beatmap else {
		return (StatusCode::NOT_FOUND, "Beatmap not found").into_response();
	};

	Json(GetRequestResponse { request, beatmap }).into_response()
}

#[derive(Deserialize)]
struct DeleteRequestBody {
	id: i64,
}

async fn delete_request(
	State(state): State<AAxumState>,
	Json(body): Json<DeleteRequestBody>,
) -> (StatusCode, &'static str) {
	match database::requests::delete_request(&state.db, body.id).await {
		Ok(_) => {
			state.state.ready_sub();
			state.state.total_sub();

			(StatusCode::OK, "OK")
		},
		_ => (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error",
		),
	}
}

async fn download_replay(
	State(state): State<AAxumState>,
	Path(id): Path<i64>,
	req: Request,
) -> impl IntoResponse {
	let Ok(request) = database::requests::get_request(&state.db, id).await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error",
		)
			.into_response();
	};

	let Some(request) = request else {
		return (StatusCode::NOT_FOUND, "Replay not found").into_response();
	};

	if !request.ready {
		return (StatusCode::BAD_REQUEST, "Replay not ready").into_response();
	}

	let filename = format!("{id}.osr");
	let filepath = format!("replays/{filename}");
	let file = ServeFile::new(filepath);
	let mut response = file.oneshot(req).await.into_response();

	response.headers_mut().insert(
		header::CONTENT_DISPOSITION,
		HeaderValue::from_str(&format!("attachment; filename=\"{filename}\"")).unwrap(),
	);

	response
}

#[derive(Deserialize)]
struct SubmitGuessBody {
	player_id: u32,
	guess: u32,
}

#[derive(Serialize)]
struct SubmitGuessResponse {
	username: String,
	rank: u32,
	guess: u32,
}

async fn submit_guess(
	State(state): State<AAxumState>,
	Json(body): Json<SubmitGuessBody>,
) -> impl IntoResponse {
	let Ok(user) = state.osu.user(body.player_id).mode(GameMode::Osu).await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected osu! API error",
		)
			.into_response();
	};

	let Some(Some(real_rank)) = user.statistics.map(|s| s.global_rank) else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"User doesn't have a rank",
		)
			.into_response();
	};

	if let Ok(Some(request)) = database::requests::get_request_by_session_player(
		&state.db,
		state.state.session_id.load(Ordering::SeqCst),
		body.player_id as u64,
	)
	.await
	{
		let _ = database::requests::mark_as_guessed_request(
			&state.db, request.id, body.guess, real_rank,
		)
		.await;

		let vod_link = match state.twitch.lock().unwrap().as_ref() {
			Some(twitch) => {
				let timestamp = Utc::now() - Duration::seconds(30);
				let link = twitch.get_link_at(timestamp);
				Some(link)
			},
			_ => None,
		};

		if let Some(link) = vod_link {
			let _ = database::requests::attach_vod_to_request(&state.db, request.id, &link).await;
		}
	}

	state.state.ready_sub();

	Json(SubmitGuessResponse {
		username: user.username.to_string(),
		rank: real_rank,
		guess: body.guess,
	})
	.into_response()
}

async fn delete_everything(State(state): State<AAxumState>) -> (StatusCode, &'static str) {
	match database::requests::delete_all_requests(&state.db).await {
		Ok(_) => {
			state.state.total_submissions.store(0, Ordering::SeqCst);
			state.state.ready_submissions.store(0, Ordering::SeqCst);

			(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Unexpected database error",
			)
		},
		_ => (StatusCode::OK, "OK"),
	}
}

pub fn router(state: AAxumState) -> Router<AAxumState> {
	Router::new()
		.route("/request", get(get_request).delete(delete_request))
		.route("/replay/{id}", get(download_replay))
		.route("/guess", post(submit_guess))
		.route("/reset", post(delete_everything))
		.layer(middleware::from_fn(auth::guesser_middleware))
		.layer(middleware::from_fn_with_state(state, auth::user_middleware))
}
