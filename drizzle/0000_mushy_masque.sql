CREATE TABLE `beatmaps` (
	`id` integer PRIMARY KEY NOT NULL,
	`beatmap_id` integer NOT NULL,
	`title` text NOT NULL,
	`artist` text NOT NULL,
	`version` text NOT NULL,
	`creator` text NOT NULL
);
--> statement-breakpoint
CREATE TABLE `requests` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`player_id` integer NOT NULL,
	`beatmap_id` integer NOT NULL,
	`client_state` text NOT NULL,
	`user_state` text NOT NULL,
	`online_state` text NOT NULL,
	`ready` integer DEFAULT false NOT NULL,
	FOREIGN KEY (`beatmap_id`) REFERENCES `beatmaps`(`id`) ON UPDATE no action ON DELETE cascade
);
