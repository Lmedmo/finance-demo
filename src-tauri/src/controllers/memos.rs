use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
//use crate::models::*;
use crate::db::get_url;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct MemoType {
    pub id: i32,
    pub name: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct MemoAlias {
    pub id: i32,
    pub memo_id: i32,
    pub alias_name: String,
    pub type_id: i32,
}

pub async fn get_memos() -> Result<Vec<MemoType>, String>{
    let memos_sql = "SELECT * FROM memos";
    let memos_query = sqlx::query_as::<_, MemoType>(memos_sql);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let memos = memos_query.fetch_all(&pool).await.expect("unable to list memo types");
    pool.close().await;

    Ok(memos)
}
pub async fn get_memo_name(id: i32)->Result<String, String>{
    let mut sql = "SELECT * FROM memos".to_owned();
    let filter_val = format!(r#" WHERE id = {id}"#);
    sql.push_str(&filter_val);
    let query = sqlx::query_as::<_, MemoType>(&sql);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let memo = query.fetch_one(&pool).await.expect("unable to list memo types");
    pool.close().await;

    Ok(memo.name)
}