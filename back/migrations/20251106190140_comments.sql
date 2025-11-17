alter table sessions
add column allow_comments integer default false not null;

alter table requests
add column comment text;