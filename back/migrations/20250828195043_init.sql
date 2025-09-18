create table beatmaps (
  id integer primary key not null,
  beatmapset_id integer not null,
  title text not null,
  artist text not null,
  version text not null,
  creator text not null
);

create table sessions (
  id integer primary key autoincrement not null,
  title text not null,
  started_at integer not null,
  ended_at integer
);

create table requests (
  id integer primary key autoincrement not null,
  player_id integer not null,
  session_id integer not null,
  beatmap_id integer not null,
  client_state text not null,
  online_state text not null,
  user_state text not null,
  ready integer default false not null,
  submitted_at integer not null,
  watched_at integer,
  guessed_rank integer,
  real_rank integer,

  foreign key (beatmap_id) references beatmaps(id)
    on update no action
    on delete cascade,
  foreign key (session_id) references sessions(id)
    on update no action
    on delete cascade,

  unique(player_id, session_id)
);