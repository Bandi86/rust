# Rust Axum backend with postgresql

youtube: https://www.youtube.com/watch?v=M0wi7V1rP4Y
github: https://github.com/AarambhDevHub/rust-backend-axum

create project: cargo new axum_auth_backend

create .env settings
add cargo.toml dependencies

install sql tool: cargo install sqlx-cli --no-default-features --features native-tls,postgres

create db: sqlx database create
migration: sqlx migrate add -r users

edit the users down sql file
edit the users up sql file create table

then sqlx migrate run

checking server:
psql -U postgres -h localhost -p 5432 -l

models.rs