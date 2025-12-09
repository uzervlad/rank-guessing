use serde::Serialize;

mod bttv;
mod ffz;
mod seventv;

#[derive(Serialize)]
pub struct Emote {
	pub name: String,
	pub url: String,
	pub zero_width: bool,
}

pub async fn fetch_emotes() -> Vec<Emote> {
	let mut emotes = vec![];
	emotes.append(
		&mut ffz::fetch_emotes().await
			.unwrap_or_default()
	);
	emotes.append(
		&mut bttv::fetch_emotes().await
			.unwrap_or_default()
	);
	emotes.append(
		&mut seventv::fetch_emotes().await
			.unwrap_or_default()
	);
	emotes
}