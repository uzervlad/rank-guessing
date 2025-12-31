use std::{convert::Infallible, fs};

use axum::{body::Body, http::{Response, header}};
use rosu_v2::model::GameMode;

use crate::{auth::UserExtension, database, replay::{Replay, analysis::{OnlineState, ReplayAnalysisTrait as _, UserState}}, routes::request::UploadMessage, state::{AAxumState, ArcAppStateTrait as _}};

macro_rules! send {
	($message:expr) => {
		Ok::<_, Infallible>(serde_json::to_string(&$message).unwrap() + "\n")
	};
}

pub fn process_replay(
	user: UserExtension,
	state: AAxumState,
	session_id: i64,
	replay_bytes: Vec<u8>,
	comment: Option<String>,
) -> Response<Body> {
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
			online_state,
			comment.clone(),
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