use eyre::Result;
use serde::Deserialize;

use super::Emote;

#[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
struct BTTVEmote {
	id: String,
	code: String,
	// image_type: String,
}

impl Into<Emote> for &BTTVEmote {
	fn into(self) -> Emote {
		Emote {
			name: self.code.clone(),
			url: format!("https://cdn.betterttv.net/emote/{}/1x", self.id),
			zero_width: false,
		}
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BTTVUser {
	channel_emotes: Vec<BTTVEmote>,
	shared_emotes: Vec<BTTVEmote>,
}

pub async fn fetch_emotes() -> Result<Vec<Emote>> {
	let global: Vec<BTTVEmote> = reqwest::get("https://api.betterttv.net/3/cached/emotes/global")
		.await?
		.json()
		.await?;

	let custom: BTTVUser = reqwest::get("https://api.betterttv.net/3/cached/users/twitch/136877209")
		.await?
		.json()
		.await?;

	let global = global.iter()
		.map(Into::into);

	let custom = custom.channel_emotes.iter()
		.chain(custom.shared_emotes.iter())
		.map(Into::into);

	Ok(global.chain(custom).collect())
}