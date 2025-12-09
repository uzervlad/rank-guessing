use axum::{Router, extract::State, response::IntoResponse, routing::get};
use reqwest::header;

use crate::state::AAxumState;

async fn root(
	State(state): State<AAxumState>,
) -> impl IntoResponse {
	let mut response = state.emotes.clone()
		.into_response();

	response.headers_mut()
		.append(header::CONTENT_TYPE, "application/json".parse().unwrap());

	response
}

pub fn router() -> Router<AAxumState> {
	Router::new()
		.route("/", get(root))
}