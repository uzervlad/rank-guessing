use axum::{
	Extension,
	extract::{Request, State},
	http::StatusCode,
	middleware::Next,
	response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use eyre::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use rosu_v2::prelude::UserExtended;
use serde::{Deserialize, Serialize};

use crate::state::AAxumState;

#[derive(Serialize, Deserialize)]
pub struct Claims {
	pub exp: i64,
	pub iat: i64,
	pub id: u64,
	pub username: String,
}

pub fn encode_jwt(secret: &str, user: &UserExtended) -> String {
	let claims = Claims {
		exp: (Utc::now() + Duration::weeks(1)).timestamp(),
		iat: Utc::now().timestamp(),
		id: user.user_id as u64,
		username: user.username.to_string(),
	};
	let key = EncodingKey::from_secret(secret.as_bytes());

	encode(&Header::default(), &claims, &key).unwrap()
}

pub fn decode_jwt(secret: &str, jwt: &str) -> Result<TokenData<Claims>> {
	let token = decode(
		jwt,
		&DecodingKey::from_secret(secret.as_bytes()),
		&Validation::default(),
	)?;

	Ok(token)
}

#[derive(Clone, Serialize)]
pub struct AuthorizedUser {
	pub id: u64,
	pub username: String,
	pub is_guesser: bool,
	pub is_admin: bool,
}

pub type UserExtension = Extension<AuthorizedUser>;

pub async fn user_middleware(
	State(state): State<AAxumState>,
	jar: CookieJar,
	mut request: Request,
	next: Next,
) -> Response {
	let Some(jwt) = jar.get("guess-token").map(|c| c.value()) else {
		return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
	};

	if jwt.is_empty() {
		return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
	}

	let Ok(data) = decode_jwt(&state.config.jwt_secret, jwt) else {
		return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
	};

	let user = AuthorizedUser {
		id: data.claims.id,
		username: data.claims.username,
		is_guesser: data.claims.id == state.config.guesser_id,
		is_admin: data.claims.id == state.config.admin_id,
	};

	request.extensions_mut().insert(user);

	next.run(request).await
}

pub async fn guesser_middleware(user: UserExtension, request: Request, next: Next) -> Response {
	if !user.is_admin && !user.is_guesser {
		return (StatusCode::FORBIDDEN, "Forbidden").into_response();
	}

	next.run(request).await
}

pub async fn admin_middleware(user: UserExtension, request: Request, next: Next) -> Response {
	if !user.is_admin {
		return (StatusCode::FORBIDDEN, "Forbidden").into_response();
	}

	next.run(request).await
}
