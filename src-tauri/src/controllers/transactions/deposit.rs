use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use crate::db::get_url;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewDeposit {
    pub type_id: i32,
    pub type_name: String,
    pub memo_id: i32,
    pub memo_name: String,
    pub date: String,
    pub amount: f32,
    pub account_id: i32,
    pub account_name: String,
    pub description: Option<String>,
    pub depositor_id: i32,
    pub depositor_name: String,
}

pub async fn add_deposit(t: NewDeposit)->String{
    let db_url = get_url();
    println!("Attempting to Add Deposit: {:?}", t);
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO transactions 
                    (type_id, type_name, memo_id, memo_name, account_id, account_name, date, depositor_id, depositor_name, description, amount) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)";
    let result = sqlx::query(&sql)
        .bind(t.type_id)
        .bind(t.type_name)
        .bind(t.memo_id)
        .bind(t.memo_name)
        .bind(t.account_id)
        .bind(t.account_name)
        .bind(t.date)
        .bind(t.depositor_id)
        .bind(t.depositor_name)
        .bind(t.description)
        .bind(t.amount)
        .execute(&pool)
        .await
        .expect("failed to add deposit to transactions");
    pool.close().await;
    let resp = format!("add_deposit: {:?}", result);
    return resp
}