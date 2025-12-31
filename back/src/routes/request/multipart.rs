use std::str::FromStr;

use axum::{Json, extract::Multipart, response::{IntoResponse, Response}};
use reqwest::StatusCode;

use crate::routes::request::UploadMessage;

pub enum UploadMultipartParseError {
	MissingType,
	InvalidType,
	ReplayBytes,
}

impl IntoResponse for UploadMultipartParseError {
	fn into_response(self) -> Response {
		(
			StatusCode::BAD_REQUEST,
			Json(UploadMessage::Error {
				message: match self {
					Self::MissingType => "Missing submission type",
					Self::InvalidType => "Invalid submission type",
					Self::ReplayBytes => "Unable to receive replay",
				}.into()
			})
		).into_response()
	}
}

pub enum UploadType {
	Replay,
	Resubmit,
	Link,
}

impl FromStr for UploadType {
	type Err = UploadMultipartParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"replay" => Ok(UploadType::Replay),
			"resubmit" => Ok(UploadType::Resubmit),
			"link" => Ok(UploadType::Link),
			_ => Err(UploadMultipartParseError::InvalidType),
		}
	}
}

pub struct UploadMultipart {
	pub _type: UploadType,
	pub replay: Option<Vec<u8>>,
	pub link: Option<String>,
	pub comment: Option<String>,
}

impl UploadMultipart {	
	pub async fn new(mut multipart: Multipart) -> Result<Self, UploadMultipartParseError> {
		let mut _type = None;
		let mut replay = None;
		let mut link = None;
		let mut comment = None;

		while let Ok(Some(field)) = multipart.next_field().await {
		let name = field.name().unwrap_or("");

		match name {
			"type" => match field.text().await {
				Ok(value) => _type = Some(value),
				_ => {},
			},
			"replay" => match field.bytes().await {
				Ok(bytes) => {
					replay = Some(bytes.to_vec());
				},
				_ => return Err(UploadMultipartParseError::ReplayBytes),
			},
			"link" => match field.text().await {
				Ok(value) => link = Some(value),
				_ => {},
			},
			"comment" => match field.text().await {
				Ok(value) => comment = Some(value),
				_ => {}
			},
			_ => {},
		}
	}

		Ok(Self {
			_type: UploadType::from_str(&_type.ok_or(UploadMultipartParseError::MissingType)?)?,
			replay,
			link,
			comment,
		})
	}
}