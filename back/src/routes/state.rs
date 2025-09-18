use std::convert::Infallible;

use axum::{
	Router,
	body::Body,
	extract::State,
	http::header,
	response::{IntoResponse, Response},
	routing::get,
};

use crate::state::{AAxumState, AppStateBroadcast};

async fn state_realtime(State(state): State<AAxumState>) -> impl IntoResponse {
	let mut rx = state.state.subscribe();

	let stream = async_stream::stream! {
		{
			let message = AppStateBroadcast::from(state.state.clone());
			yield Ok::<_, Infallible>(serde_json::to_string(&message).unwrap() + "\n");
		}

		while let Ok(message) = rx.recv().await {
			yield Ok::<_, Infallible>(serde_json::to_string(&message).unwrap() + "\n");
		}
	};

	Response::builder()
		.header(header::CONTENT_TYPE, "application/x-ndjson")
		.body(Body::from_stream(stream))
		.unwrap()
}

pub fn router() -> Router<AAxumState> {
	Router::new().route("/", get(state_realtime))
}
