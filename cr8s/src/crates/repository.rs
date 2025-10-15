use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
  crates::model::{Crate, NewCrate},
  schema::crates,
};

pub struct CrateRepository;

impl CrateRepository {
  /// CRUD Read 単一の取得 dieselを使った時の書き方
  pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
    crates::table.find(id).get_result(c).await // これでqueryを投げる
  }

  /// CRUD Read 複数習得 dieselを使った時の書き方
  pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
    // 複数取得する場合
    crates::table.limit(limit).get_results(c).await
  }

  /// CRUD insertの参考に
  pub async fn create(c: &mut AsyncPgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
    diesel::insert_into(crates::table)
      .values(new_crate)
      .get_result(c)
      .await
  }

  /// update CRUD dieselでudpateをする時の参考に
  pub async fn update(c: &mut AsyncPgConnection, cr: Crate) -> QueryResult<Crate> {
    diesel::update(crates::table.find(cr.id)) // 更新する行を絞り込む (今回はfindで代用)
      .set((
        // 何をどう更新するかを指定
        crates::name.eq(cr.name),
        crates::version.eq(cr.version),
        crates::description.eq(cr.description),
        crates::code.eq(cr.code),
      ))
      .get_result(c)
      .await
  }

  /// Delete CRUD diesel で deleteとする時の参考に
  pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(crates::table.find(id)).execute(c).await
  }
}
