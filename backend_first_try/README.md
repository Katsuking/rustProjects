### crates

```sh
cargo add actix-web json serde surrealdb uuid validator async-trait derive_more
cargo build
```

Cargo.toml

```toml
[dependencies]
actix-web = "4.5.1"
async-trait = "0.1.80"
derive_more = "0.99.17"
json = "0.12.4"
serde = {version = "1.0.197",  features = ["derive"]}
surrealdb = "1.4.0"
uuid = "1.8.0"
validator = {version = "0.18.1", features = ["derive"]}
```

補足

- actix-web: Web フレームワーク

[actix](https://actix.rs/)

- json serde: json を扱うため serde で serialize/deseialize

- surrealdb: rust 製の DB

- async-trait derive_more: コードの保守性をあげるため

### ホットリロード for backend

```sh
cargo install cargo-watch # run this once
cargo watch -x run -c -q
```

### docker compose

Spin up a multi-node development environment

```sh
docker compose up --pull always -d
```

Spin up a multi-node development environment with monitoring

```sh
docker compose up --pull always --profile monitoring -d
```
