use crate::schema::*; // schemaはすべてインポートしておく
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Rustacean {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub created_at: NaiveDateTime, // schema.rs の Timestamp をホバーするといくつか候補になる型を確認できる
}

#[derive(Insertable, Debug)]
#[diesel(table_name=rustaceans)] // 新規作成時はrustaceansテーブルに作成するように紐付け
pub struct NewRustacean {
  pub name: String,
  pub email: String,
}
