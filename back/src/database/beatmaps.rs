use eyre::Result;
use rosu_v2::prelude::BeatmapExtended;
use serde::Serialize;
use sqlx::{SqlitePool, prelude::FromRow};

#[derive(FromRow, Serialize)]
pub struct DbBeatmap {
	pub id: i64,
	pub beatmapset_id: i64,
	pub title: String,
	pub artist: String,
	pub version: String,
	pub creator: String,
}

pub async fn get_beatmap(pool: &SqlitePool, id: i64) -> Result<Option<DbBeatmap>> {
	let beatmap = sqlx::query_as::<_, DbBeatmap>(
		r#"
		select * from beatmaps
		where id = $1
	"#,
	)
	.bind(id)
	.fetch_optional(pool)
	.await?;

	Ok(beatmap)
}

pub async fn update_beatmap(pool: &SqlitePool, beatmap: &BeatmapExtended) -> Result<()> {
	let id = beatmap.map_id;
	let beatmapset_id = beatmap.mapset_id;
	let artist = beatmap
		.mapset
		.as_ref()
		.map(|s| s.artist.as_str())
		.unwrap_or_default();
	let title = beatmap
		.mapset
		.as_ref()
		.map(|s| s.title.as_str())
		.unwrap_or_default();
	let creator = beatmap
		.mapset
		.as_ref()
		.map(|s| s.creator_name.as_str())
		.unwrap_or_default();
	let version = beatmap.version.as_str();

	let _ = sqlx::query(
		r#"
    insert into beatmaps
      (id, beatmapset_id, artist, title, creator, version)
      values ($1, $2, $3, $4, $5, $6)
    on conflict(id) do
      update set
        artist = $3,
        title = $4,
        creator = $5,
        version = $6
  "#,
	)
	.bind(id)
	.bind(beatmapset_id)
	.bind(artist)
	.bind(title)
	.bind(creator)
	.bind(version)
	.execute(pool)
	.await?;

	Ok(())
}
