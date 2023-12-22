use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use crate::controllers::*;
use crate::db::get_url;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewTransaction {
    pub type_id: i32,
    pub type_name: String,
    pub memo_id: i32,
    pub memo_name: String,
    pub date: String,
    pub amount: f32,
    pub account_id: i32,
    pub account_name: String,
    pub description: Option<String>,
    pub to_from_account: Option<i32>,
    pub to_from_acct_name: Option<String>,
    pub merchant_id: Option<i32>,
    pub merchant_name: Option<String>,
    pub depositor_id: Option<i32>,
    pub depositor_name: Option<String>,
    pub category_id: Option<i32>,
    pub category_name: Option<String>,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Transaction {
    pub id: i32,
    pub type_id: i32,
    pub type_name: String,
    pub memo_id: i32,
    pub memo_name: String,
    pub date: String,
    pub amount: f32,
    pub account_id: i32,
    pub account_name: String,
    pub description: Option<String>,
    pub to_from_account: Option<i32>,
    pub to_from_acct_name: Option<String>,
    pub merchant_id: Option<i32>,
    pub merchant_name: Option<String>,
    pub depositor_id: Option<i32>,
    pub depositor_name: Option<String>,
    pub category_id: Option<i32>,
    pub category_name: Option<String>,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Transactions {
    records: Vec<Transaction>,
    types: Vec<TransactionType>,
    memos: Vec<MemoType>,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct TransactionType {
    pub id: i32,
    pub name: String,
}

// Read
pub async fn get_transactions() -> Result<Transactions, String>{
    let sql = "SELECT id, type_id, type_name, memo_id, memo_name, account_id, account_name, to_from_account, to_from_acct_name, date, merchant_id, merchant_name, depositor_id, depositor_name, description, category_id, category_name, ROUND(amount, 2) as amount FROM transactions;";
    let query = sqlx::query_as::<_, Transaction>(sql);
    
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let transactions = query.fetch_all(&pool).await.expect("unable to list transactions");
    pool.close().await;

    let types = get_transaction_types().await.unwrap();
    let memos = get_memos().await.unwrap();

    let resp = Transactions { records: transactions, types, memos };

    Ok(resp)
}
pub async fn get_transaction_types() -> Result<Vec<TransactionType>, String>{
    let sql = "SELECT * FROM transaction_types";
    let query = sqlx::query_as::<_, TransactionType>(&sql);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let t_types = query.fetch_all(&pool).await.expect("unable to list transaction types");
    pool.close().await;

    Ok(t_types)
}

// Edit
pub async fn edit_transaction(t: NewTransaction, id: i32)->String{
    //println!("Attempting to update transaction... Record: {:?}", t);
    let sql = "UPDATE transactions SET 
                   type_id = $1, type_name = $2,
                   memo_id = $3, memo_name = $4,
                   account_id = $5, account_name = $6,
                   to_from_account = $7, to_from_acct_name = $8,
                   date = $9,
                   merchant_id = $10, merchant_name = $11,
                   depositor_id = $12, depositor_name = $13,
                   description = $14,
                   category_id = $15, category_name = $16,
                   amount = $17
               WHERE id = $18;";
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let result = sqlx::query(&sql)
        .bind(t.type_id)
        .bind(t.type_name)
        .bind(t.memo_id)
        .bind(t.memo_name)
        .bind(t.account_id)
        .bind(t.account_name)
        .bind(t.to_from_account)
        .bind(t.to_from_acct_name)
        .bind(t.date)
        .bind(t.merchant_id)
        .bind(t.merchant_name)
        .bind(t.depositor_id)
        .bind(t.depositor_name)
        .bind(t.description)
        .bind(t.category_id)
        .bind(t.category_name)
        .bind(t.amount)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to update transaction");
    pool.close().await;
    let resp = format!("edit_transaction: {:?}", result);
    return resp
}
pub async fn update_to_from_accts(tfa_id: i32, tfa_name: &String, desc: &String){
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");

    let mut sql = "UPDATE transactions SET to_from_account = ".to_owned();
    let filter_val = format!(r#"{tfa_id}, to_from_acct_name = '{tfa_name}' WHERE description LIKE '{desc}%'"#);
    sql.push_str(&filter_val);

    let _result = sqlx::query(&sql)
        .execute(&pool)
        .await
        .expect("failed to update transactions' to_from_account values");
    pool.close().await;
}

// Delete
pub async fn delete_transaction(rec: i32)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "DELETE FROM transactions WHERE id = $1";
    let result = sqlx::query(&sql)
        .bind(rec)
        .execute(&pool)
        .await
        .expect("failed to delete transaction");
    pool.close().await;
    let resp = format!("delete_transaction: {:?}", result);
    return resp
}