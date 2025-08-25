import { sqliteTable, integer, text, numeric } from 'drizzle-orm/sqlite-core';

export enum ClientState {
	/** Stable replay */
	Stable = 'stable',
	/** Lazer replay with stable mods */
	LazerDefault = 'lazer',
	/** Lazer replay with lazer-specific mods */
	LazerMods = 'lazer-mods',
}

export enum UserState {
	/** User in replay matches authorized user */
	SameUser = 'same_user',
	/** User in replay doesn't match authorized user */
	OtherUser = 'other_user',
	/** User id is missing in replay (offline play) */
	NotPresent = 'not_present',
}

export enum OnlineState {
	/** Score is valid and available online */
	Available = 'available',
	/** Score id is present in replay but not available online */
	Unavailable = 'unavailable',
	/** Score id is missing in replay (maybe offline play) */
	NotPresent = 'not_present',
}

export const requests = sqliteTable('requests', {
	id: integer('id').primaryKey({ autoIncrement: true }),
	player_id: integer('player_id').notNull(),
	beatmap_id: integer('beatmap_id').notNull()
		.references(() => beatmaps.id, { onDelete: "cascade" }),
	client_state: text().notNull()
		.$type<ClientState>(),
	user_state: text().notNull()
		.$type<UserState>(),
	online_state: text().notNull()
		.$type<OnlineState>(),
	ready: integer({ mode: 'boolean' }).notNull()
		.default(false),
	watched_at: integer({ mode: 'timestamp' }),
});

export const beatmaps = sqliteTable('beatmaps', {
	id: integer('id').primaryKey(),
	beatmapset_id: integer('beatmap_id').notNull(),
	title: text('title').notNull(),
	artist: text('artist').notNull(),
	version: text('version').notNull(),
	creator: text('creator').notNull(),
});
