use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

//use crate::models::*;
//use crate::utilities::*;
//use crate::controllers::*;

use crate::db::get_url;

use super::NewAccount;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewCreditCard {
    pub name: String,
    pub bank_id: i32,
    pub user_id: i32,
    pub type_id: i32,
    pub initial_balance: f32,
    pub credit_limit: f32,
    pub due_date: String,
    pub interest_rate: f32,
}

pub async fn add_credit_card(a: NewCreditCard)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO accounts 
                    (name, bank_id, user_id, type_id, initial_balance, credit_limit, due_date, interest_rate) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
    let result = sqlx::query(&sql)
        .bind(a.name)
        .bind(a.bank_id)
        .bind(a.user_id)
        .bind(a.type_id)
        .bind(a.initial_balance)
        .bind(a.credit_limit)
        .bind(a.due_date)
        .bind(a.interest_rate)
        .execute(&pool)
        .await
        .expect("failed to add_credit_card");
    pool.close().await;
    let resp = format!("add_credit_card: {:?}", result);
    return resp
}
pub async fn edit_credit_card(vals: NewAccount, id: i32)->String{
    let mut sql = "UPDATE accounts SET ".to_owned();

    let namesql = format!(r#"name = '{}', "#, vals.name);
    let banksql = format!("bank_id = {}, ", vals.bank_id);
    let usersql = format!("user_id = {}, ", vals.user_id);
    let typesql = format!("type_id = {}, ", vals.type_id);
    let ibalsql = format!("initial_balance = {}, ", vals.initial_balance);
    let climsql = format!("credit_limit = {:?}, ", vals.credit_limit.unwrap());
    let datesql = format!("due_date = {:?}, ", vals.due_date.unwrap());
    let ratesql = format!("interest_rate = {} ", vals.initial_balance);
    let filter_val = format!("WHERE id = {}; ", id);

    sql.push_str(&namesql);
    sql.push_str(&banksql);
    sql.push_str(&usersql);
    sql.push_str(&typesql);
    sql.push_str(&ibalsql);
    sql.push_str(&climsql);
    sql.push_str(&datesql);
    sql.push_str(&ratesql);
    sql.push_str(&filter_val);

    println!("sql: {}", sql);

    // let sql = "UPDATE accounts SET 
    //                name = $1,
    //                bank_id = $2,
    //                user_id = $3,
    //                type_id = $4,
    //                initial_balance = $5,
    //                credit_limit = $6,
    //                due_date = $7,
    //                interest_rate = $8
    //            WHERE id = $9;";
    
    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let result = sqlx::query(&sql)
        // .bind(vals.name)
        // .bind(vals.bank_id)
        // .bind(vals.user_id)
        // .bind(vals.type_id)
        // .bind(vals.initial_balance)
        // .bind(vals.credit_limit)
        // .bind(vals.due_date)
        // .bind(vals.interest_rate)
        // .bind(id)
        .execute(&pool)
        .await
        .expect("failed to update credit card");
    pool.close().await;
    let resp = format!("edit_account: {:?}", result);
    return resp
}