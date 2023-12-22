use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use crate::db::get_url;

use super::NewAccount;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewInvestmentAccount {
    pub name: String,
    pub bank_id: i32,
    pub user_id: i32,
    pub type_id: i32,
    pub initial_balance: f32,
}

pub async fn add_investment_account(a: NewInvestmentAccount)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO accounts 
                    (name, bank_id, user_id, type_id, initial_balance) 
                VALUES ($1, $2, $3, $4, $5)";
    let result = sqlx::query(&sql)
        .bind(a.name)
        .bind(a.bank_id)
        .bind(a.user_id)
        .bind(a.type_id)
        .bind(a.initial_balance)
        .execute(&pool)
        .await
        .expect("failed to add_investment_account");
    pool.close().await;
    let resp = format!("add_investment_account: {:?}", result);
    return resp
}
pub async fn edit_invest_acct(vals: NewAccount, id: i32)->String{
    let sql = "UPDATE accounts SET 
                   name = $1,
                   bank_id = $2,
                   user_id = $3,
                   type_id = $4,
                   initial_balance = $5
               WHERE id = $6;";

    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let result = sqlx::query(&sql)
        .bind(vals.name)
        .bind(vals.bank_id)
        .bind(vals.user_id)
        .bind(vals.type_id)
        .bind(vals.initial_balance)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to update investment account");
    pool.close().await;
    let resp = format!("edit_account: {:?}", result);
    return resp
}