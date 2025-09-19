use std::{convert::Infallible, fs, sync::atomic::Ordering};

use axum::{
	Json, Router,
	body::Body,
	extract::{DefaultBodyLimit, Multipart, State},
	http::{Response, StatusCode, header},
	middleware,
	response::IntoResponse,
	routing::get,
};
use rosu_v2::model::GameMode;
use serde::Serialize;
use tower_http::limit::RequestBodyLimitLayer;

use crate::{
	auth::{self, UserExtension},
	database::{self, requests::DbRequest},
	replay::{
		Replay,
		analysis::{OnlineState, ReplayAnalysisTrait, UserState},
	},
	state::{AAxumState, ArcAppStateTrait},
};

macro_rules! send {
	($message:expr) => {
		Ok::<_, Infallible>(serde_json::to_string(&$message).unwrap() + "\n")
	};
}

#[derive(Serialize)]
struct RequestResponse {
	request: Option<DbRequest>,
}

async fn get_current_request(
	user: UserExtension,
	State(state): State<AAxumState>,
) -> impl IntoResponse {
	let session_id = state.state.session_id.load(Ordering::SeqCst);

	let Ok(request) =
		database::requests::get_request_by_session_player(&state.db, session_id, user.id).await
	else {
		return (StatusCode::INTERNAL_SERVER_ERROR, "Unknown database error").into_response();
	};

	Json(RequestResponse { request }).into_response()
}

async fn get_requests(user: UserExtension, State(state): State<AAxumState>) -> impl IntoResponse {
	let Ok(requests) = database::requests::get_requests_by_player(&state.db, user.id).await else {
		return (StatusCode::INTERNAL_SERVER_ERROR, "Unknown database error").into_response();
	};

	Json(requests).into_response()
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum UploadMessage {
	Info { message: String },
	Error { message: String },
	Done { id: i64 },
}

async fn upload_replay(
	user: UserExtension,
	State(state): State<AAxumState>,
	mut multipart: Multipart,
) -> impl IntoResponse {
	let session_id = state.state.session_id.load(Ordering::SeqCst);

	match database::requests::get_request_by_session_player(&state.db, session_id, user.id).await {
		Ok(Some(_)) => {
			return (
				StatusCode::BAD_REQUEST,
				Json(UploadMessage::Error {
					message: "You already submitted a replay".into(),
				}),
			)
				.into_response();
		},
		Err(_) => {
			return (
				StatusCode::BAD_REQUEST,
				Json(UploadMessage::Error {
					message: "Unknown database error".into(),
				}),
			)
				.into_response();
		},
		_ => {},
	}

	if state.state.session_id.load(Ordering::SeqCst) == 0 {
		return (
			StatusCode::SERVICE_UNAVAILABLE,
			Json(UploadMessage::Error {
				message: "Submissions are currently disabled".into(),
			}),
		)
			.into_response();
	}

	let mut replay_bytes = None;

	while let Ok(Some(field)) = multipart.next_field().await {
		let name = field.name().unwrap_or("");

		if name == "replay" {
			match field.bytes().await {
				Ok(bytes) => {
					replay_bytes = Some(bytes.to_vec());
				},
				_ => {
					return (
						StatusCode::BAD_REQUEST,
						Json(UploadMessage::Error {
							message: "Unable to receive replay".into(),
						}),
					)
						.into_response();
				},
			}
		}
	}

	let Some(replay_bytes) = replay_bytes else {
		return (
			StatusCode::BAD_REQUEST,
			Json(UploadMessage::Error {
				message: "No replay file".into(),
			}),
		)
			.into_response();
	};

	let stream = async_stream::stream! {
	  yield send! { UploadMessage::Info { message: "Parsing replay...".into() } };

	  let Ok((_, mut replay)) = Replay::decode(&replay_bytes) else {
			yield send! { UploadMessage::Error { message: "Unable to parse replay file".into() } };
			return;
	  };

	  if replay.ruleset() != 0 {
			yield send! { UploadMessage::Error { message: "Only standard allowed".into() } };
			return;
	  }

	  yield send! { UploadMessage::Info { message: "Analyzing replay...".into() } };

	  let client_state = replay.client_state();

	  yield send! { UploadMessage::Info { message: "Checking beatmap...".into() } };

	  let Ok(beatmap) = state.osu.beatmap().checksum(replay.beatmap_hash()).await else {
			yield send! { UploadMessage::Error { message: "Unable to fetch beatmap".into() } };
			return;
	  };

	  let Ok(_) = database::beatmaps::update_beatmap(&state.db, &beatmap).await else {
			yield send! { UploadMessage::Error { message: "Unexpected database error".into() } };
			return;
	  };

	  yield send! { UploadMessage::Info { message: "Checking online score...".into() } };

	  let mut online_state = OnlineState::NotPresent;
	  let mut online_user_id = None;

	  'online: {
		if let Some(online_id) = replay.online_id() {
		  online_state = OnlineState::Unavailable;

		  let Ok(a) = (match replay.is_legacy_score() {
				true => state.osu.score(online_id as u64).mode(GameMode::Osu).await,
				false => state.osu.score(online_id as u64).await,
		  }) else { break 'online; };

		  online_state = OnlineState::Available;
		  online_user_id = Some(a.user_id as u64);
		}
	  };

	  let user_state = match online_user_id.or(replay.user_id()) {
			Some(id) if id == user.id => UserState::SameUser,
			Some(_) => UserState::OtherUser,
			_ => UserState::NotPresent,
	  };

	  yield send! { UploadMessage::Info { message: "Saving replay...".into() } };

	  let Ok(request_id) = database::requests::create_request(
			&state.db,
			user.id,
			beatmap.map_id,
			session_id,
			client_state,
			user_state,
			online_state
	  ).await else {
			yield send! { UploadMessage::Error { message: "Unexpected database error".into() } };
			return;
	  };

		state.state.total_add();

	  replay.anonymize(request_id);
	  let Ok(replay_bytes) = replay.encode() else {
			yield send! { UploadMessage::Error { message: "Unable to encode replay file".into() } };
			return;
	  };

	  let Ok(_) = fs::write(format!("replays/{}.osr", request_id), replay_bytes) else {
			yield send! { UploadMessage::Error { message: "Unexpected IO error".into() } };
			return;
	  };

	  let Ok(_) = database::requests::ready_request(&state.db, request_id).await else {
			yield send! { UploadMessage::Error { message: "Unexpected database error".into() } };
			return;
	  };

	  yield send! { UploadMessage::Done { id: request_id } };

		state.state.ready_add();
	};

	Response::builder()
		.header(header::CONTENT_TYPE, "application/x-ndjson")
		.body(Body::from_stream(stream))
		.unwrap()
}

async fn delete_request(
	user: UserExtension,
	State(state): State<AAxumState>,
) -> (StatusCode, &'static str) {
	let session_id = state.state.session_id.load(Ordering::SeqCst);

	let request =
		match database::requests::get_request_by_session_player(&state.db, session_id, user.id)
			.await
		{
			Ok(Some(request)) => request,
			Ok(_) => return (StatusCode::NOT_FOUND, "No request found"),
			_ => {
				return (
					StatusCode::INTERNAL_SERVER_ERROR,
					"Unexpected database error",
				);
			},
		};

	let Ok(_) = database::requests::delete_request(&state.db, request.id).await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error",
		);
	};

	if request.ready {
		state.state.ready_sub();
	}

	state.state.total_sub();

	(StatusCode::OK, "OK")
}

pub fn router(state: AAxumState) -> Router<AAxumState> {
	Router::new()
		.route(
			"/",
			get(get_current_request)
				.post(upload_replay)
				.delete(delete_request),
		)
		.route("/list", get(get_requests))
		.layer(DefaultBodyLimit::disable())
		.layer(RequestBodyLimitLayer::new(1024 * 1024))
		.layer(middleware::from_fn_with_state(state, auth::user_middleware))
}
