use std::collections::HashMap;

use eyre::Result;
use serde::Deserialize;

use super::Emote;

#[derive(Deserialize)]
struct FFZEmote {
	name: String,
	urls: HashMap<u32, String>,
}

impl Into<Emote> for &FFZEmote {
	fn into(self) -> Emote {
		Emote {
			name: self.name.clone(),
			url: self.urls.get(&2).cloned().unwrap_or("".into()),
			zero_width: false,
		}
	}
}

#[derive(Deserialize)]
struct FFZEmoteSet {
	emoticons: Vec<FFZEmote>,
}

#[derive(Deserialize)]
struct FFZRoom {
	sets: HashMap<u32, FFZEmoteSet>
}

pub async fn fetch_emotes() -> Result<Vec<Emote>> {
	let global: FFZRoom = reqwest::get("https://api.frankerfacez.com/v1/set/global")
		.await?
		.json()
		.await?;

	let custom: FFZRoom = reqwest::get("https://api.frankerfacez.com/v1/room/id/136877209")
		.await?
		.json()
		.await?;

	let global = global.sets.get(&3)
		.map(|s| s.emoticons.iter())
		.unwrap_or_default()
		.map(Into::into);

	let custom = custom.sets.get(&197984)
		.map(|s| s.emoticons.iter())
		.unwrap_or_default()
		.map(Into::into);

	Ok(global.chain(custom).collect())
}