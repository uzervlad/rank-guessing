use std::{fs, sync::atomic::Ordering};

use axum::{
	Json, Router,
	extract::{DefaultBodyLimit, Multipart, State},
	http::StatusCode,
	middleware,
	response::IntoResponse,
	routing::get,
};
use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::digit1, combinator::{map, map_res}, sequence::preceded};
use rosu_v2::model::GameMode;
use serde::{Deserialize, Serialize};
use tower_http::limit::RequestBodyLimitLayer;

use crate::{
	auth::{self, UserExtension}, database::{self, requests::DbRequest},
	routes::request::{multipart::{UploadMultipart, UploadType}, upload::process_replay}, state::{AAxumState, ArcAppStateTrait}
};

mod upload;
mod multipart;

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

async fn get_last_request(
	user: UserExtension,
	State(state): State<AAxumState>,
) -> impl IntoResponse {
	let Ok(request) =
		database::requests::get_last_request_by_player(&state.db, user.id).await
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
pub enum UploadMessage {
	Info { message: String },
	Error { message: String },
	Done { id: i64 },
}

async fn upload_replay(
	user: UserExtension,
	State(state): State<AAxumState>,
	multipart: Multipart,
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

	let multipart = match UploadMultipart::new(multipart).await {
		Ok(m) => m,
		Err(e) => return e.into_response(),
	};

	if let Some(comment) = &multipart.comment
		&& comment.len() > 100
	{
		return (
			StatusCode::BAD_REQUEST,
			Json(UploadMessage::Error {
				message: "Comment too long".into()
			})
		)
			.into_response()
	}

	match multipart._type {
		UploadType::Replay => {
			let Some(replay) = multipart.replay else {
				return (
					StatusCode::BAD_REQUEST,
					Json(UploadMessage::Error {
						message: "No replay file".into(),
					}),
				)
					.into_response();
			};

			process_replay(user, state, session_id, replay, multipart.comment)
		},
		UploadType::Resubmit => {
			let last_request = match database::requests::get_last_request_by_player(&state.db, user.id).await {
				Ok(Some(request)) => request,
				Ok(_) => return (
					StatusCode::NOT_FOUND,
					Json(UploadMessage::Error {
						message: "No replay found".into()
					}),
				).into_response(),
				Err(_) => return (
					StatusCode::INTERNAL_SERVER_ERROR,
					Json(UploadMessage::Error {
						message: "Unknown database error".into()
					}),
				).into_response(),
			};

			if last_request.watched_at.is_some() {
				return (
					StatusCode::BAD_REQUEST,
					Json(UploadMessage::Error {
						message: "Request has been watched".into()
					}),
				).into_response()
			}

			let Ok(replay) = fs::read(format!("./replays/{}.osr", last_request.id)) else {
				return (
					StatusCode::INTERNAL_SERVER_ERROR,
					Json(UploadMessage::Error {
						message: "Unable to read replay file".into()
					}),
				).into_response()
			};

			process_replay(user, state, session_id, replay, multipart.comment)
		},
		UploadType::Link => {
			let Some(link) = multipart.link else {
				return (
					StatusCode::BAD_REQUEST,
					Json(UploadMessage::Error {
						message: "No score link provided".into(),
					}),
				).into_response()
			};

			let parse_result = {
				fn parse_score_id(input: &str) -> IResult<&str, u64> {
					map_res(digit1, str::parse)
						.parse(input)
				}

				fn parse_mode(input: &str) -> IResult<&str, GameMode> {
					map(
						alt((
							tag("osu"),
							tag("taiko"),
							tag("fruits"),
							tag("mania"),
						)),
						|s: &str| match s {
							"osu" => GameMode::Osu,
							"taiko" => GameMode::Taiko,
							"fruits" => GameMode::Catch,
							"mania" => GameMode::Mania,
							_ => unreachable!()
						}
					).parse(input)
				}

				preceded(
					tag("https://osu.ppy.sh/scores/"),
					alt((
						map(parse_score_id, |id| (id, None)),
						map(
							(parse_mode, tag("/"), parse_score_id),
							|(mode, _, id)| (id, Some(mode))
						)
					))
				).parse(&link)
			};

			let Ok((_, (score_id, mode))) = parse_result else {
				return (
					StatusCode::BAD_REQUEST,
					Json(UploadMessage::Error {
						message: "Invalid score link".into(),
					}),
				).into_response()
			};

			let Ok(replay) = match mode {
				Some(mode) => state.osu.replay_raw(score_id).mode(mode),
				_ => state.osu.replay_raw(score_id),
			}.await else {
				return (
					StatusCode::BAD_REQUEST,
					Json(UploadMessage::Error {
						message: "Unable to download replay".into(),
					}),
				).into_response()
			};

			process_replay(user, state, session_id, replay, multipart.comment)
		},
	}
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

#[derive(Deserialize)]
struct ChangeCommentBody {
	comment: String,
}

async fn change_comment(
	user: UserExtension,
	State(state): State<AAxumState>,
	Json(body): Json<ChangeCommentBody>,
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

	let Ok(_) = database::requests::change_request_comment(&state.db, request.id, body.comment).await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unexpected database error"
		);
	};

	(StatusCode::OK, "OK")
}

pub fn router(state: AAxumState) -> Router<AAxumState> {
	Router::new()
		.route(
			"/",
			get(get_current_request)
				.post(upload_replay)
				.delete(delete_request)
				.patch(change_comment),
		)
		.route(
			"/last",
			get(get_last_request)
		)
		.route("/list", get(get_requests))
		.layer(DefaultBodyLimit::disable())
		.layer(RequestBodyLimitLayer::new(1024 * 1024))
		.layer(middleware::from_fn_with_state(state, auth::user_middleware))
}
