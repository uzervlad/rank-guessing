use eyre::Result;
use serde::Deserialize;

use super::Emote;

#[derive(Deserialize)]
struct SevenTvEmoteDataHost {
	url: String,
}

#[derive(Deserialize)]
struct SevenTvEmoteData {
	host: SevenTvEmoteDataHost,
}

#[derive(Deserialize)]
struct SevenTvEmote {
	name: String,
	flags: u8,
	data: SevenTvEmoteData,
}

impl Into<Emote> for &SevenTvEmote {
	fn into(self) -> Emote {
		Emote {
			name: self.name.clone(),
			url: format!("https:{}/1x.avif", self.data.host.url),
			zero_width: (self.flags & 1) == 1,
		}
	}
}

#[derive(Deserialize)]
struct SevenTvEmoteSet {
	emotes: Vec<SevenTvEmote>,
}

#[derive(Deserialize)]
struct SevenTvUser {
	emote_set: SevenTvEmoteSet,
}

pub async fn fetch_emotes() -> Result<Vec<Emote>> {
	let global: SevenTvEmoteSet = reqwest::get("https://7tv.io/v3/emote-sets/global")
		.await?
		.json()
		.await?;

	let custom: SevenTvUser = reqwest::get("https://7tv.io/v3/users/twitch/136877209")
		.await?
		.json()
		.await?;

	let global = global.emotes.iter()
		.map(Into::into);

	let custom = custom.emote_set.emotes.iter()
		.map(Into::into);

	Ok(global.chain(custom).collect())
}