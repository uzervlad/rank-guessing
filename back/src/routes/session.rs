use std::sync::atomic::Ordering;

use axum::{
	Json, Router,
	extract::State,
	http::StatusCode,
	middleware,
	response::IntoResponse,
	routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::{
	auth,
	database::{self, sessions::DbSession},
	state::{AAxumState, ArcAppStateTrait},
};

#[derive(Serialize)]
struct SessionResponse {
	session: Option<DbSession>,
}

async fn get_current_session(State(state): State<AAxumState>) -> impl IntoResponse {
	let session_id = state.state.session_id.load(Ordering::SeqCst);
	if session_id == 0 {
		return Json(SessionResponse { session: None }).into_response()
	}

	let Ok(session) = database::sessions::get_current_session(&state.db).await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error",
		)
			.into_response();
	};

	Json(SessionResponse { session }).into_response()
}

async fn get_sessions(State(state): State<AAxumState>) -> impl IntoResponse {
	let Ok(sessions) = database::sessions::get_sessions(&state.db).await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error",
		)
			.into_response();
	};

	Json(sessions).into_response()
}

#[derive(Deserialize)]
struct StartSessionBody {
	name: String,
}

#[derive(Serialize)]
struct StartSessionResponse {
	id: i64,
}

async fn start_session(
	State(state): State<AAxumState>,
	Json(body): Json<StartSessionBody>,
) -> impl IntoResponse {
	let session_id = state.state.session_id.load(Ordering::SeqCst);
	if session_id != 0 {
		return (StatusCode::BAD_REQUEST, "Ongoing session already exists").into_response();
	}

	match database::sessions::get_current_session(&state.db).await {
		Ok(Some(_)) => {
			return (StatusCode::BAD_REQUEST, "Ongoing session already exists").into_response();
		},
		Err(_) => {
			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Unexpected database error",
			)
				.into_response();
		},
		_ => {},
	}

	let Ok(id) = database::sessions::create_session(&state.db, body.name).await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error",
		)
			.into_response();
	};

	state.state.set_session_id(id);

	Json(StartSessionResponse { id }).into_response()
}

async fn end_session(State(state): State<AAxumState>) -> impl IntoResponse {
	let Ok(_) = database::sessions::end_session(&state.db).await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error",
		)
			.into_response();
	};

	state.state.reset();

	"OK".into_response()
}

#[derive(Deserialize)]
struct RenameSessionBody {
	name: String,
}

async fn rename_session(
	State(state): State<AAxumState>,
	Json(body): Json<RenameSessionBody>
) -> (StatusCode, &'static str) {
	match database::sessions::rename_session(&state.db, body.name).await {
		Ok(_) => (StatusCode::OK, "OK"),
		_ => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected database error"),
	}
}

pub fn router(state: AAxumState) -> Router<AAxumState> {
	let protected = Router::new()
		.route("/", post(start_session).delete(end_session).patch(rename_session))
		.route("/list", get(get_sessions))
		.layer(middleware::from_fn(auth::guesser_middleware))
		.layer(middleware::from_fn_with_state(state, auth::user_middleware));

	let non_protected = Router::new()
		.route("/", get(get_current_session));

	protected.merge(non_protected)
}
