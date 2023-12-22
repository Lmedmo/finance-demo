use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use crate::db::get_url;

use super::NewAccount;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewCheckingAccount {
    pub name: String,
    pub bank_id: i32,
    pub user_id: i32,
    pub type_id: i32,
    pub initial_balance: f32,
    pub account_number: String,
    pub routing_number: String,
}
pub async fn add_checking_account(a: NewCheckingAccount)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO accounts 
                    (name, bank_id, user_id, type_id, initial_balance, account_number, routing_number) 
                VALUES ($1, $2, $3, $4, $5, $6, $7)";
    let result = sqlx::query(&sql)
        .bind(a.name)
        .bind(a.bank_id)
        .bind(a.user_id)
        .bind(a.type_id)
        .bind(a.initial_balance)
        .bind(a.account_number)
        .bind(a.routing_number)
        .execute(&pool)
        .await
        .expect("failed to add_checking_account");
    pool.close().await;
    let resp = format!("add_checking_account: {:?}", result);
    return resp
}
pub async fn edit_checking_acct(vals: NewAccount, id: i32)->String{
    let sql = "UPDATE accounts SET 
                   name = $1,
                   bank_id = $2,
                   user_id = $3,
                   type_id = $4,
                   initial_balance = $5,
                   account_number = $6,
                   routing_number = $7
               WHERE id = $8;";

    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let result = sqlx::query(&sql)
        .bind(vals.name)
        .bind(vals.bank_id)
        .bind(vals.user_id)
        .bind(vals.type_id)
        .bind(vals.initial_balance)
        .bind(vals.account_number)
        .bind(vals.routing_number)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to update checking account");
    pool.close().await;
    let resp = format!("edit_account: {:?}", result);
    return resp
}