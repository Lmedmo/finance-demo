use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use crate::db::get_url;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewExpense {
    pub type_id: i32,
    pub type_name: String,
    pub memo_id: i32,
    pub memo_name: String,
    pub date: String,
    pub amount: f32,
    pub account_id: i32,
    pub account_name: String,
    pub description: Option<String>,
    pub merchant_id: i32,
    pub merchant_name: String,
    pub category_id: i32,
    pub category_name: String,
}

pub async fn add_expense(t: NewExpense)->String{
    let db_url = get_url();
    println!("Attempting to Add Expense: {:?}", t);
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO transactions 
                    (type_id, type_name, memo_id, memo_name, account_id, account_name, date, merchant_id, merchant_name, description, category_id, category_name, amount) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)";
    let result = sqlx::query(&sql)
        .bind(t.type_id)
        .bind(t.type_name)
        .bind(t.memo_id)
        .bind(t.memo_name)
        .bind(t.account_id)
        .bind(t.account_name)
        .bind(t.date)
        .bind(t.merchant_id)
        .bind(t.merchant_name)
        .bind(t.description)
        .bind(t.category_id)
        .bind(t.category_name)
        .bind(t.amount)
        .execute(&pool)
        .await
        .expect("failed to add expense to transactions");
    pool.close().await;
    let resp = format!("add_expense: {:?}", result);
    return resp
}