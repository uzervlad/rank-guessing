use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Encode, Sqlite, Type, sqlite::SqliteTypeInfo};

use crate::replay::Replay;

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClientState {
	Stable,
	Lazer,
	LazerMods,
}

impl Display for ClientState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Stable => "stable",
				Self::Lazer => "lazer",
				Self::LazerMods => "lazer-mods",
			}
		)
	}
}

impl TryFrom<String> for ClientState {
	type Error = ();

	fn try_from(value: String) -> Result<Self, Self::Error> {
		match value.as_str() {
			"stable" => Ok(Self::Stable),
			"lazer" => Ok(Self::Lazer),
			"lazer-mods" => Ok(Self::LazerMods),
			_ => Err(()),
		}
	}
}

impl Type<Sqlite> for ClientState {
	fn type_info() -> SqliteTypeInfo {
		<String as Type<Sqlite>>::type_info()
	}
}

impl<'q> Encode<'q, Sqlite> for ClientState {
	fn encode_by_ref(
		&self,
		buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
	) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
		<String as Encode<'q, Sqlite>>::encode(self.to_string(), buf)
	}
}

impl<'r> Decode<'r, Sqlite> for ClientState {
	fn decode(
		value: <Sqlite as sqlx::Database>::ValueRef<'r>,
	) -> Result<Self, sqlx::error::BoxDynError> {
		let value = <String as Decode<'r, Sqlite>>::decode(value)?;

		Self::try_from(value.clone())
			.map_err(|_| format!("invalid value `{value}` for enum ClientState").into())
	}
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OnlineState {
	Available,
	Unavailable,
	NotPresent,
}

impl Display for OnlineState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Available => "available",
				Self::Unavailable => "unavailable",
				Self::NotPresent => "not-present",
			}
		)
	}
}

impl TryFrom<String> for OnlineState {
	type Error = ();

	fn try_from(value: String) -> Result<Self, Self::Error> {
		match value.as_str() {
			"available" => Ok(Self::Available),
			"unavailable" => Ok(Self::Unavailable),
			"not-present" => Ok(Self::NotPresent),
			_ => Err(()),
		}
	}
}

impl Type<Sqlite> for OnlineState {
	fn type_info() -> SqliteTypeInfo {
		<String as Type<Sqlite>>::type_info()
	}
}

impl<'q> Encode<'q, Sqlite> for OnlineState {
	fn encode_by_ref(
		&self,
		buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
	) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
		<String as Encode<'q, Sqlite>>::encode(self.to_string(), buf)
	}
}

impl<'r> Decode<'r, Sqlite> for OnlineState {
	fn decode(
		value: <Sqlite as sqlx::Database>::ValueRef<'r>,
	) -> Result<Self, sqlx::error::BoxDynError> {
		let value = <String as Decode<'r, Sqlite>>::decode(value)?;

		Self::try_from(value.clone())
			.map_err(|_| format!("invalid value `{value}` for enum ClientState").into())
	}
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserState {
	SameUser,
	OtherUser,
	NotPresent,
}

impl Display for UserState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::SameUser => "same-user",
				Self::OtherUser => "other-user",
				Self::NotPresent => "not-present",
			}
		)
	}
}

impl TryFrom<String> for UserState {
	type Error = ();

	fn try_from(value: String) -> Result<Self, Self::Error> {
		match value.as_str() {
			"same-user" => Ok(Self::SameUser),
			"other-user" => Ok(Self::OtherUser),
			"not-present" => Ok(Self::NotPresent),
			_ => Err(()),
		}
	}
}

impl Type<Sqlite> for UserState {
	fn type_info() -> SqliteTypeInfo {
		<String as Type<Sqlite>>::type_info()
	}
}

impl<'q> Encode<'q, Sqlite> for UserState {
	fn encode_by_ref(
		&self,
		buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
	) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
		<String as Encode<'q, Sqlite>>::encode(self.to_string(), buf)
	}
}

impl<'r> Decode<'r, Sqlite> for UserState {
	fn decode(
		value: <Sqlite as sqlx::Database>::ValueRef<'r>,
	) -> Result<Self, sqlx::error::BoxDynError> {
		let value = <String as Decode<'r, Sqlite>>::decode(value)?;

		Self::try_from(value.clone())
			.map_err(|_| format!("invalid value `{value}` for enum ClientState").into())
	}
}

pub trait ReplayAnalysisTrait {
	fn client_state(&self) -> ClientState;
}

const STABLE_MODS: [&str; 12] = [
	"EZ", "NF", "HT", "HR", "SD", "PF", "DT", "NC", "HD", "FL", "RX", "AP",
];

#[derive(Deserialize)]
struct ScoreInfoMod {
	acronym: String,
	settings: Option<Value>,
}

impl ScoreInfoMod {
	fn is_stable(&self) -> bool {
		STABLE_MODS.iter().any(|m| m == &self.acronym) && self.settings.is_none()
	}
}

impl ReplayAnalysisTrait for Replay {
	fn client_state(&self) -> ClientState {
		if self.is_legacy_score() {
			return ClientState::Stable;
		}

		let mods = self
			.score_info
			.as_ref()
			.and_then(|info| info.get("mods"))
			.and_then(|m| serde_json::from_value::<Vec<ScoreInfoMod>>(m.clone()).ok());

		match mods {
			Some(mods) if mods.iter().all(ScoreInfoMod::is_stable) => ClientState::Lazer,
			Some(_) => ClientState::LazerMods,
			_ => ClientState::Lazer,
		}
	}
}
