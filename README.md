### rust

99% の react, 1% の rust 構成でも大抵の人はこう言います。
"blazingly fast!"

実際は rust だけじゃないから全然早くないよねって言われても言い続けます...

### motivations

[discord switching from go to rust](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)

### cargo になれる

プロジェクトを新規作成して、Crates を追加する

```sh
cargo new <you name projectname>
cd <you name projectname>
cargo add <crates like json serde etc>
cargo build
```

### Cargo.toml

node.js でいうところの package.json のようなもの

例えば、

```sh
cargo add actix-web json serde surrealdb uuid validator async-trait derive_more
cargo build
```

を実行した場合は下記のようになります。

```toml
[dependencies]
actix-web = "4.5.1"
async-trait = "0.1.80"
derive_more = "0.99.17"
json = "0.12.4"
serde = "1.0.197"
surrealdb = "1.4.0"
uuid = "1.8.0"
validator = "0.18.1"
```

さらに、パッケージの機能を有効化/無効化したい場合

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

`cargo build`を再度実行

### surrealdb

rust 製の DB

[surrealdb](https://github.com/surrealdb/surrealdb)

[docker](https://github.com/surrealdb/docker.surrealdb.com)

### ホットリロード for backend

web 系開発しているなら必須

```sh
cargo install cargo-watch # run this once
cargo watch -x run -c -q
```

### rust の モジュール

例えば、下記のように、`src/models/sushi.rs`を作成したときに
`models.rs`を作成することで`models` dir を認識させる

```
src
├── main.rs
├── models
│   └── sushi.rs
└── models.rs

```

読み込ませるためには、以下のように追記が必要

main.rs

```rust
mod models;
```

models.rs

```rust
pub mod sushi
```

### なんか関数が見つからないとき

```
use actix_web::{
    get, patch, post,
    web::{Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
```

`web::Path`もあるし、`dev::Path`もあって、
間違った方を使ってると、補完が当然効かない...
