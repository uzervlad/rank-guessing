use axum::{Json, Router, extract::State, middleware, routing::post};
use serde::Deserialize;

use crate::{auth, database, state::AAxumState};

#[derive(Deserialize)]
struct SetAdditionalNotesBody {
  request_id: i64,
  note: String,
}

async fn set_additional_notes(
  State(state): State<AAxumState>,
  Json(body): Json<SetAdditionalNotesBody>,
) {
  let _ = database::requests::set_additional_notes(
    &state.db,
    body.request_id,
    if body.note.is_empty() {
      None
    } else {
      Some(body.note)
    }
  ).await;
}

pub fn router(state: AAxumState) -> Router<AAxumState> {
  Router::new()
    .route("/notes", post(set_additional_notes))
    .layer(middleware::from_fn(auth::admin_middleware))
    .layer(middleware::from_fn_with_state(state, auth::user_middleware))
}