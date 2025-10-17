### permssion error

```sh
echo "UID=$(id -u)" >> .env && echo "GID=$(id -g)" >> .env
```

同じユーザーを持つ

```docker-compose
user: "${UID}:${GID}"
```

### diesel cli

```sh
docker compose exec app diesel setup # docker-composeに記載したとおり接続できるかの確認 DATABASE_URLが正しいことの確認
docker compose exec app diesel migration list # migration リストの表示

docker compose exec app diesel migration generate create_rustaceans # migration ファイルの作成
docker compose exec app diesel migration generate create_crates # migration ファイルの作成
# sql ファイル作成後
docker compose exec app diesel migration run # マイグレーションの実行
docker compose exec app diesel migration revert # マイグレーション rollback
```

### openssl error

```
warning: openssl-sys@0.9.110: Could not find directory of OpenSSL installation,
and this `-sys` crate cannot proceed without this knowledge.
If OpenSSL is installed and this crate had trouble finding it,
you can set the `OPENSSL_DIR` environment variable for the compilation process.
See stderr section below for further information.
error: failed to run custom build command for `openssl-sys v0.9.110`
```

必要なパッケージのインストール

```sh
sudo apt install libssl-dev
```

### rust-analyzer

この設定を vscode の setting.json に入れてたらディレクトリ削除したときに
rust-analyzer がバグるので入れないほうがいい

```json
// .vscode/settings.json
{
  // Make sure the old, deleted path is NOT in this list.
  "rust-analyzer.linkedProjects": [
    // "/mnt/su650_adata/development/rustProjects/everyday_rust/day1/temp_converter/Cargo.toml", // <--- DELETE THIS LINE
    "/path/to/your/current/project/Cargo.toml"
  ]
}
```

### Blocking Code in Async Context

Rust’s async runtime is cooperative.
Blocking operations such as long file reads with `std::fs::read` or
expensive computations stall the entire executor thread.

Developers sometimes forget to offload blocking work to a separate thread pool.

```rs
use axum::{routing::get, Router}; use tokio::task;   async fn handler() -> String {
  // Wrong: This blocks the async runtime
  let data = std::fs::read_to_string("large.txt").unwrap();
  // Correct: Offload with `spawn_blocking`
  let data = task::spawn_blocking(|| std::fs::read_to_string("large.txt"))
    .await
    .unwrap()
    .unwrap();
  format!("Read {} bytes", data.len()) }
```
