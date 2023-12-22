use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

//use crate::models::*;
use crate::utilities::*;
use crate::controllers::*;
use crate::db::get_url;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewAccount {
    pub name: String,
    pub bank_id: i32,
    pub user_id: i32,
    pub type_id: i32,
    pub initial_balance: f32,
    pub credit_limit: Option<f32>,
    pub due_date: Option<String>,
    pub interest_rate: Option<f32>,
    pub compound_frequency: Option<i32>,
    pub period_unit: Option<String>,
    pub account_username: Option<String>,
    pub account_number: Option<String>,
    pub routing_number: Option<String>,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct AccountRec {
    pub id: i32,
    pub name: String,
    pub bank_id: i32,
    pub user_id: i32,
    pub type_id: i32,
    pub initial_balance: f32,
    pub credit_limit: Option<f32>,
    pub due_date: Option<String>,
    pub interest_rate: Option<f32>,
    pub compound_frequency: Option<i32>,
    pub period_unit: Option<String>,
    pub account_username: Option<String>,
    pub account_number: Option<String>,
    pub routing_number: Option<String>,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub bank_id: i32,
    pub user_id: i32,
    pub type_id: i32,
    pub initial_balance: f32,
    pub credit_limit: Option<f32>,
    pub due_date: Option<String>,
    pub interest_rate: Option<f32>,
    pub compound_frequency: Option<i32>,
    pub period_unit: Option<String>,
    pub account_username: Option<String>,
    pub account_number: Option<String>,
    pub routing_number: Option<String>,
    pub balance: f64,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct AccountType {
    pub id: i32,
    pub name: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct CompoundFreqOpt {
    pub id: i32,
    pub name: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct PeriodUnitType {
    pub id: i32,
    pub name: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Accounts {
    records: Vec<Account>,
    types: Vec<AccountType>,
    cfreqs: Vec<CompoundFreqOpt>,
    punits: Vec<PeriodUnitType>,
    banks: Vec<Bank>,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Bank {
    pub id: i32,
    pub name: String,
    pub icon: String,
    pub icon_color: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct PaymentTransferAlias {
    pub id: i32,
    pub account_id: i32,
    pub to_from_account: i32,
    pub alias_name: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewPaymentTransferAlias {
    pub account_id: i32,
    pub to_from_account: i32,
    pub alias_name: String,
}
// ------------------------ end of types ----------------------------- //

/* Read */
pub async fn get_accounts() -> Result<Accounts, String>{
    let accts_sql = "SELECT * FROM accounts WHERE id > 1";
    let accts_query = sqlx::query_as::<_, AccountRec>(accts_sql);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let accts = accts_query.fetch_all(&pool).await.expect("unable to list accounts");
    pool.close().await;

    let types = get_account_types().await.unwrap();
    let cfreqs = get_compound_freq_opts().await.unwrap();
    let punits = get_period_unit_types().await.unwrap();
    let banks = get_banks().await.unwrap();

    let mut records: Vec<Account> = Vec::new();

    for acct in accts {
        let calcbal = get_account_bal(acct.id).await.unwrap();
        let initbal = acct.initial_balance;
        let bal = calcbal + initbal as f64;
        let val = Account {
            id: acct.id,
            name: acct.name,
            bank_id: acct.bank_id,
            user_id: acct.user_id,
            type_id: acct.type_id,
            initial_balance: acct.initial_balance,
            credit_limit: acct.credit_limit,
            due_date: acct.due_date,
            interest_rate: acct.interest_rate,
            compound_frequency: acct.compound_frequency,
            period_unit: acct.period_unit,
            account_username: acct.account_username,
            account_number: acct.account_number,
            routing_number: acct.routing_number,
            balance: bal
        };
        records.push(val);
    }

    let resp = Accounts {records, types, cfreqs, punits, banks};

    Ok(resp)
}
pub async fn get_account_types() -> Result<Vec<AccountType>, String>{
    let sql = "SELECT * FROM account_types";
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let query = sqlx::query_as::<_, AccountType>(&sql);
    let result = query.fetch_all(&pool).await.expect("unable to get account types");
    pool.close().await;
    Ok(result)
}
pub async fn get_compound_freq_opts() -> Result<Vec<CompoundFreqOpt>, String>{
    let sql = "SELECT * FROM comp_frequencies";
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let query = sqlx::query_as::<_, CompoundFreqOpt>(&sql);
    let result = query.fetch_all(&pool).await.expect("unable to get compound frequency opts");
    pool.close().await;
    Ok(result)
}
pub async fn get_period_unit_types() -> Result<Vec<PeriodUnitType>, String>{
    let sql = "SELECT * FROM period_units";
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let query = sqlx::query_as::<_, PeriodUnitType>(&sql);
    let result = query.fetch_all(&pool).await.expect("unable to get period unit types");
    pool.close().await;
    Ok(result)
}
pub async fn get_banks() -> Result<Vec<Bank>, String>{
    let sql = "SELECT * FROM banks";
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let query = sqlx::query_as::<_, Bank>(&sql);
    let result = query.fetch_all(&pool).await.expect("unable to get banks");
    pool.close().await;
    Ok(result)
}
pub async fn get_account_info(id: i32) -> Result<AccountRec, String>{
    let mut sql = "SELECT * FROM accounts".to_owned();
    let filter_val = format!(" WHERE id = {}", id);
    if id != 0 {
        sql.push_str(&filter_val);
    }
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let query = sqlx::query_as::<_, AccountRec>(&sql);
    let result = query.fetch_one(&pool).await.expect("unable to find account");
    pool.close().await;
    Ok(result)
}

/* Create */
pub async fn add_payment_transfer_alias(a: PaymentTransferAlias, tfa_name: &String){
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO payment_transfer_aliases 
                    (account_id, to_from_account, alias_name) 
                VALUES ($1, $2, $3)";
    let _result = sqlx::query(&sql)
        .bind(a.account_id)
        .bind(a.to_from_account)
        .bind(&a.alias_name)
        .execute(&pool)
        .await
        .expect("failed to add_payment_transfer_alias");
    pool.close().await;
    update_to_from_accts(a.to_from_account, tfa_name, &a.alias_name).await;
}
/* Update */
pub async fn edit_account(vals: NewAccount, id: i32)->String{
    //println!("vals: {:?}", vals);
    let mut resp: String = "".to_string();
    
    if vals.type_id == 1 {
        resp = edit_credit_card(vals, id).await;
    } else if vals.type_id == 2 {
        resp = edit_mobile_acct(vals, id).await;
    } else if vals.type_id == 3 {
        resp = edit_checking_acct(vals, id).await;
    } else if vals.type_id == 4 {
        resp = edit_savings_acct(vals, id).await;
    } else if vals.type_id == 5 {
        resp = edit_invest_acct(vals, id).await;
    } else if vals.type_id == 6 {
        resp = "Invalid Account type".to_string();
    }
    
    return resp
}

/* Delete */
pub async fn delete_account(id: i32)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "DELETE from transactions WHERE account_id = $1;
               DELETE from accounts WHERE id = $1;";
    let result = sqlx::query(&sql)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to delete account and transactions");
    pool.close().await;
    let resp = format!("delete_account: {:?}", result);
    return resp
}