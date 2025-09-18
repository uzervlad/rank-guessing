# rank guessing v2

A complete rewrite using Rust for the backend

### Backend

```sh
# install sqlx-cli
$ cargo install sqlx-cli

# create database and apply migrations
$ sqlx database create
$ sqlx migrate run

# start the backend
$ cargo run
```

### Frontend

```sh
bun i
bun run dev
```
