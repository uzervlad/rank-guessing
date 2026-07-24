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

	let (ffz, bttv, seventv) = tokio::join!(
		ffz::fetch_emotes(),
		bttv::fetch_emotes(),
		seventv::fetch_emotes(),
	);

	emotes.append(&mut ffz.unwrap_or_default());
	emotes.append(&mut bttv.unwrap_or_default());
	emotes.append(&mut seventv.unwrap_or_default());

	emotes
}