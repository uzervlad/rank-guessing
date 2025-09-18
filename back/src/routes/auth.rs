use axum::{
	Json, Router,
	extract::{Query, State},
	http::StatusCode,
	middleware,
	response::{IntoResponse, Redirect},
	routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use rosu_v2::{OsuBuilder, prelude::Scopes};
use serde::Deserialize;

use crate::{
	auth::{self, AuthorizedUser, UserExtension, encode_jwt},
	state::AAxumState,
};

async fn osu_login(State(state): State<AAxumState>) -> Redirect {
	Redirect::to(&state.config.osu.to_auth_url())
}

#[derive(Deserialize)]
struct CallbackQuery {
	code: String,
}

async fn osu_callback(
	State(state): State<AAxumState>,
	jar: CookieJar,
	Query(CallbackQuery { code }): Query<CallbackQuery>,
) -> impl IntoResponse {
	let Ok(osu_client) = OsuBuilder::new()
		.client_id(state.config.osu.client_id)
		.client_secret(&state.config.osu.client_secret)
		.with_authorization(code, &state.config.osu.redirect_uri, Scopes::Identify)
		.build()
		.await
	else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unable to initialize API client",
		)
			.into_response();
	};

	let Ok(me) = osu_client.own_data().await else {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unable to fetch authorizing user",
		)
			.into_response();
	};

	let token = encode_jwt(&state.config.jwt_secret, &me);

	let mut cookie = Cookie::new("guess-token", token);
	cookie.set_path("/");
	cookie.set_http_only(false);

	let new_jar = jar.add(cookie);

	(new_jar, Redirect::to(&state.config.frontend_url)).into_response()
}

async fn me(user: UserExtension) -> Json<AuthorizedUser> {
	Json(user.0)
}

pub fn router(state: AAxumState) -> Router<AAxumState> {
	Router::new()
		.route("/", get(osu_login))
		.route("/callback", get(osu_callback))
		.route(
			"/me",
			get(me).route_layer(middleware::from_fn_with_state(state, auth::user_middleware)),
		)
}
