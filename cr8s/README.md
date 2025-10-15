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
