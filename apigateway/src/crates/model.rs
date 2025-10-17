use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};

use crate::schema::crates;

#[derive(Queryable, Debug)]
pub struct Crate {
  pub id: i32,
  pub rustacean_id: i32,
  pub code: String,
  pub name: String,
  pub version: String,
  pub description: Option<String>,
  pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name=crates)] // 新規作成時はcratesテーブルに作成するように紐付け
pub struct NewCrate {
  pub rustacean_id: i32,
  pub code: String,
  pub name: String,
  pub version: String,
  pub description: Option<String>,
}
