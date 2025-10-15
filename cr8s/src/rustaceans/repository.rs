use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
  rustaceans::model::{NewRustacean, Rustacean},
  schema::rustaceans,
};

/// 名前の通り、自動生成されたschema.rsのrustacenasテーブルに対するCRUDを実装する
pub struct RustaceanRepository;

impl RustaceanRepository {
  /// CRUD Read 単一の取得 dieselを使った時の書き方
  pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
    rustaceans::table.find(id).get_result(c).await // これでqueryを投げる
  }

  /// CRUD Read 複数習得 dieselを使った時の書き方
  pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
    // 複数取得する場合
    rustaceans::table.limit(limit).get_results(c).await
  }

  /// CRUD insertの参考に
  pub async fn create(
    c: &mut AsyncPgConnection,
    new_rustacean: NewRustacean,
  ) -> QueryResult<Rustacean> {
    diesel::insert_into(rustaceans::table)
      .values(new_rustacean)
      .get_result(c)
      .await
  }

  /// update CRUD dieselでudpateをする時の参考に
  pub async fn update(c: &mut AsyncPgConnection, rustacean: Rustacean) -> QueryResult<Rustacean> {
    diesel::update(rustaceans::table.find(rustacean.id)) // 更新する行を絞り込む (今回はfindで代用)
      .set((
        // 何をどう更新するかを指定
        // 2つのレコードしかupdateしないので
        rustaceans::name.eq(rustacean.name),
        rustaceans::email.eq(rustacean.email),
      ))
      .get_result(c)
      .await
  }

  /// Delete CRUD diesel で deleteとする時の参考に
  pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(rustaceans::table.find(id)).execute(c).await
  }
}
