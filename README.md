# GraphQL example in rust

Uses [diesel](http://diesel.rs/guides) to interact with the database.
Uses [rocket](https://rocket.rs/guide/) to provide the webserver (for now, will become tokio-minihttp).

The GraphQL tool used is juniper.

## Setup

```sh
cargo install diesel_cli
```

### Create database

```sh
diesel setup
diesel migrate run
```
