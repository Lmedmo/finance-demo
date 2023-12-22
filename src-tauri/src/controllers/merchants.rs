use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

use crate::db::get_url;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewMerchant {
    pub name: String,
    pub icon: String,
    pub icon_color: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewMerchantCategory {
    pub merchant_id: i32,
    pub category_id: i32,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Merchant {
    pub id: i32,
    pub name: String,
    pub icon: String,
    pub icon_color: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Merchants {
    records: Vec<Merchant>,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct MerchantCategory {
    pub id: i32,
    pub merchant_id: i32,
    pub category_id: i32,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct MerchantAlias {
    pub id: i32,
    pub merchant_id: i32,
    pub alias_name: String,
}
pub async fn get_merchants() -> Result<Merchants, String>{
    let sql = "SELECT * FROM merchants";
    let query = sqlx::query_as::<_, Merchant>(sql);

    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let merchants = query.fetch_all(&pool).await.expect("unable to list merchants");
    pool.close().await;

    let resp = Merchants { records: merchants };

    Ok(resp)
}
pub async fn get_merchant_categories(filtered: bool, _merch_id: i32, _cat_id: i32) -> Result<Vec<MerchantCategory>, String>{
    let mut sql = "SELECT * FROM merchant_categories".to_owned();

    if filtered {
        let filter_val = format!(r#" WHERE merchant_id = {_merch_id} AND category_id = {_cat_id}"#);
        sql.push_str(&filter_val);
    }

    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let query = sqlx::query_as::<_, MerchantCategory>(&sql);
    let mcats = query.fetch_all(&pool).await.expect("unable to list merchant categories");
    pool.close().await;

    Ok(mcats)
}
pub async fn get_merchant_name(id: i32) -> Result<String, String>{
    let mut sql = "SELECT * FROM merchants".to_owned();
    let filter_val = format!(r#" WHERE id = {id}"#);
    sql.push_str(&filter_val);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let query = sqlx::query_as::<_, Merchant>(&sql);
    let merchant = query.fetch_one(&pool).await.expect("unable to get merchant");
    pool.close().await;

    let resp = merchant.name;

    Ok(resp)
}

pub async fn add_merchant(m: NewMerchant)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO merchants 
                    (name, icon, icon_color) 
                VALUES ($1, $2, $3)";
    let result = sqlx::query(&sql)
        .bind(m.name)
        .bind(m.icon)
        .bind(m.icon_color)
        .execute(&pool)
        .await
        .expect("failed to add merchant");
    pool.close().await;
    let resp = format!(": {:?}", result);
    return resp
}
pub async fn add_merchant_category(m: NewMerchantCategory){
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO merchant_categories 
                    (category_id, merchant_id) 
                VALUES ($1, $2)";
    let _result = sqlx::query(&sql)
        .bind(m.category_id)
        .bind(m.merchant_id)
        .execute(&pool)
        .await
        .expect("failed to add merchant category");
    pool.close().await;
}
pub async fn edit_merchant(m: NewMerchant, id: i32)->String{
    let sql = "UPDATE merchants SET
                    name = $1,
                    icon = $2,
                    icon_color = $3
               WHERE id = $4;";
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let result = sqlx::query(&sql)
        .bind(m.name)
        .bind(m.icon)
        .bind(m.icon_color)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to update merchant");
    pool.close().await;
    let resp = format!("edit_merchant: {:?}", result);
    return resp
}