use std::io::Write;

use eyre::Result;
use nom::{
	IResult,
	number::complete::{le_i32, le_i64, le_u8, le_u16, le_u64},
};
use serde_json::{Map, Value};
use xz2::{
	stream::{LzmaOptions, Stream},
	write::{XzDecoder, XzEncoder},
};

pub mod analysis;

const FIRST_LAZER_VERSION: i32 = 30000000;

fn nom_fail(input: &[u8]) -> nom::Err<nom::error::Error<&[u8]>> {
	nom::Err::Failure::<nom::error::Error<&[u8]>>(nom::error::Error::new(
		input,
		nom::error::ErrorKind::Fail,
	))
}

#[derive(Debug)]
pub struct Replay {
	ruleset: u8,
	game_version: i32,
	beatmap_hash: String,
	username: String,
	replay_hash: String,
	n300: u16,
	n100: u16,
	n50: u16,
	nmiss: u16,
	ngeki: u16,
	nkatu: u16,
	total_score: i32,
	max_combo: u16,
	perfect: u8,
	raw_mods: i32,
	life_data: String,
	date: u64,
	compressed_replay: Vec<u8>,
	online_id: i64,
	score_info: Option<Map<String, Value>>,
}

impl Replay {
	pub fn decode(input: &[u8]) -> IResult<&[u8], Replay> {
		let (input, ruleset) = le_u8(input)?;
		let (input, game_version) = le_i32(input)?;

		let (input, beatmap_hash) = read_string(input)?;
		let (input, username) = read_string(input)?;
		let (input, replay_hash) = read_string(input)?;

		let (input, n300) = le_u16(input)?;
		let (input, n100) = le_u16(input)?;
		let (input, n50) = le_u16(input)?;
		let (input, nmiss) = le_u16(input)?;
		let (input, ngeki) = le_u16(input)?;
		let (input, nkatu) = le_u16(input)?;

		let (input, total_score) = le_i32(input)?;
		let (input, max_combo) = le_u16(input)?;
		let (input, perfect) = le_u8(input)?;
		let (input, raw_mods) = le_i32(input)?;

		let (input, life_data) = read_string(input)?;

		let (input, date) = le_u64(input)?;

		let (mut input, compressed_replay) = read_byte_array(input)?;

		let online_id = if game_version >= 20140721 {
			let id;
			(input, id) = le_i64(input)?;
			id
		} else if game_version >= 20121008 {
			let id;
			(input, id) = le_i32(input)?;
			id as i64
		} else {
			-1
		};

		let score_info = if game_version >= 30000001 {
			let (input, compressed_score_info) = read_byte_array(input)?;

			let mut decoder = XzDecoder::new_stream(
				Vec::new(),
				Stream::new_lzma_decoder(u64::MAX).map_err(|_| nom_fail(input))?,
			);
			decoder
				.write_all(&compressed_score_info)
				.map_err(|_| nom_fail(input))?;
			let score_info_bytes = decoder.finish().map_err(|_| nom_fail(input))?;
			Some(serde_json::from_slice(&score_info_bytes).map_err(|_| nom_fail(input))?)
		} else {
			None
		};

		let replay = Replay {
			ruleset,
			game_version,
			beatmap_hash,
			username,
			replay_hash,
			n300,
			n100,
			n50,
			nmiss,
			ngeki,
			nkatu,
			total_score,
			max_combo,
			perfect,
			raw_mods,
			life_data,
			date,
			compressed_replay,
			online_id,
			score_info,
		};

		Ok((input, replay))
	}

	pub fn encode(&self) -> Result<Vec<u8>> {
		let mut bytes = Vec::new();

		bytes.extend(self.ruleset.to_le_bytes());
		bytes.extend(self.game_version.to_le_bytes());

		write_string(&mut bytes, &self.beatmap_hash);
		write_string(&mut bytes, &self.username);
		write_string(&mut bytes, &self.replay_hash);

		bytes.extend(self.n300.to_le_bytes());
		bytes.extend(self.n100.to_le_bytes());
		bytes.extend(self.n50.to_le_bytes());
		bytes.extend(self.nmiss.to_le_bytes());
		bytes.extend(self.ngeki.to_le_bytes());
		bytes.extend(self.nkatu.to_le_bytes());

		bytes.extend(self.total_score.to_le_bytes());
		bytes.extend(self.max_combo.to_le_bytes());
		bytes.extend(self.perfect.to_le_bytes());
		bytes.extend(self.raw_mods.to_le_bytes());

		write_string(&mut bytes, &self.life_data);

		bytes.extend(self.date.to_le_bytes());

		write_byte_array(&mut bytes, &self.compressed_replay);

		if self.game_version >= 20140721 {
			bytes.extend(self.online_id.to_le_bytes());
		} else if self.game_version >= 20121008 {
			bytes.extend((self.online_id as i32).to_le_bytes());
		}

		if let Some(score_info) = &self.score_info {
			let score_info_bytes = serde_json::to_vec(score_info)?;

			// let mut encoder = XzEncoder::new(Vec::new(), 5);
			let mut encoder = XzEncoder::new_stream(
				Vec::new(),
				Stream::new_lzma_encoder(&LzmaOptions::new_preset(5)?)?,
			);
			encoder.write_all(&score_info_bytes)?;
			let compressed_score_info = encoder.finish()?;
			write_byte_array(&mut bytes, &compressed_score_info);
		}

		Ok(bytes)
	}

	pub fn is_legacy_score(&self) -> bool {
		self.game_version < FIRST_LAZER_VERSION
	}

	pub fn ruleset(&self) -> u8 {
		self.ruleset
	}

	pub fn beatmap_hash(&self) -> &str {
		&self.beatmap_hash
	}

	pub fn user_id(&self) -> Option<u64> {
		match &self.score_info {
			Some(score_info) => match score_info.get("user_id") {
				Some(value) if value.is_number() => value.as_u64(),
				_ => None,
			},
			_ => None,
		}
	}

	pub fn online_id(&self) -> Option<i64> {
		match self.online_id {
			-1 => match &self.score_info {
				Some(score_info) => match score_info.get("online_id") {
					Some(value) if value.is_number() => value.as_i64(),
					_ => None,
				},
				_ => None,
			},
			id => Some(id),
		}
	}

	pub fn anonymize(&mut self, id: i64) {
		self.username = id.to_string();
		if let Some(score_info) = &mut self.score_info {
			score_info.remove("user_id");
		}
	}
}

fn read_string(input: &[u8]) -> IResult<&[u8], String> {
	let (input, flag) = le_u8(input)?;

	if flag != 0x0b {
		return Ok((input, String::new()));
	}

	let (mut input, len) = read_uleb128(input)?;

	let len = len as usize;

	let s = str::from_utf8(&input[..len])
		.map(|s| s.into())
		.map_err(|_| nom_fail(input))?;

	input = &input[len..];

	Ok((input, s))
}

fn read_uleb128(mut input: &[u8]) -> IResult<&[u8], u128> {
	let mut value: u128 = 0;
	let mut shift: u128 = 0;
	let mut byte: u8;

	loop {
		(input, byte) = le_u8(input)?;
		value |= ((byte & 0x7f) as u128) << shift;
		shift += 7;

		if (byte & 0x80) == 0 {
			break Ok((input, value));
		}
	}
}

fn read_byte_array(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
	let (input, len) = le_i32(input)?;
	let len = len as usize;

	if input.len() < len {
		Err(nom::Err::Failure::<nom::error::Error<&[u8]>>(
			nom::error::Error::new(input, nom::error::ErrorKind::Fail),
		))?;
	}

	let array = Vec::from(&input[..len]);

	Ok((&input[len..], array))
}

fn write_string(bytes: &mut Vec<u8>, s: &str) {
	bytes.push(0x0b);
	write_uleb128(bytes, s.len() as u128);
	bytes.extend_from_slice(s.as_bytes());
}

fn write_uleb128(bytes: &mut Vec<u8>, mut value: u128) {
	loop {
		let mut byte = (value & 0x7f) as u8;
		value >>= 7;

		if value != 0 {
			byte |= 0x80;
		}

		bytes.push(byte);

		if value == 0 {
			break;
		}
	}
}

fn write_byte_array(bytes: &mut Vec<u8>, array: &[u8]) {
	bytes.extend((array.len() as i32).to_le_bytes());
	bytes.extend_from_slice(array);
}
