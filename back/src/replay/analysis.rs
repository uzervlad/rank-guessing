use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Encode, Sqlite, Type, sqlite::SqliteTypeInfo};

use crate::replay::Replay;

macro_rules! sqlite_enum {
	($name:ident, $($key:ident => $value:literal),+) => {
		#[derive(Clone, Copy, Debug, Serialize)]
		#[serde(rename_all = "kebab-case")]
		pub enum $name {
			$(
				$key,
			)*
		}

		impl Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(
					f,
					"{}",
					match self {
						$(
							Self::$key => $value,
						)*
					}
				)
			}
		}

		impl TryFrom<String> for $name {
			type Error = ();

			fn try_from(value: String) -> Result<Self, Self::Error> {
				match value.as_str() {
					$(
						$value => Ok(Self::$key),
					)*
					_ => Err(()),
				}
			}
		}

		impl Type<Sqlite> for $name {
			fn type_info() -> SqliteTypeInfo {
				<String as Type<Sqlite>>::type_info()
			}
		}

		impl<'q> Encode<'q, Sqlite> for $name {
			fn encode_by_ref(
				&self,
				buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
			) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
				<String as Encode<'q, Sqlite>>::encode(self.to_string(), buf)
			}
		}

		impl<'r> Decode<'r, Sqlite> for $name {
			fn decode(
				value: <Sqlite as sqlx::Database>::ValueRef<'r>,
			) -> Result<Self, sqlx::error::BoxDynError> {
				let value = <String as Decode<'r, Sqlite>>::decode(value)?;

				Self::try_from(value.clone())
					.map_err(|_| format!("invalid value `{}` for enum {}", value, stringify!($name)).into())
			}
		}
	}
}

sqlite_enum! {
	ClientState,
	Stable => "stable",
	Lazer => "lazer",
	LazerMods => "lazer-mods"
}

sqlite_enum! {
	OnlineState,
	Available => "available",
	Unavailable => "unavailable",
	NotPresent => "not-present"
}

sqlite_enum! {
	UserState,
	SameUser => "same-user",
	OtherUser => "other-user",
	NotPresent => "not-present"
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
