use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct App {
    pub id: i32,
    pub theme: String,
    pub font: String,
}

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct CSVRecord {
    pub date: String,
    pub description: String,
    pub amount: f32,
}

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct DBresp {
    pub changes: u64,
    pub last_insert_rowid: i64,
}

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct CheckKVP {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AppleCardRec {
    #[serde(rename = "Transaction Date")]
    pub transaction_date: String,
    #[serde(rename = "Clearing Date")]
    pub clearing_date: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Merchant")]
    pub merchant: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Type")]
    pub t_type: String,
    #[serde(rename = "Amount (USD)")]
    pub amount: f32,
    #[serde(rename = "Purchased By")]
    pub purchased_by: String,
}

#[derive(Debug, Deserialize)]
pub struct ChaseRec {
    #[serde(rename = "Transaction Date")]
    pub transaction_date: String,
    #[serde(rename = "Post Date")]
    pub clearing_date: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Type")]
    pub t_type: String,
    #[serde(rename = "Amount")]
    pub amount: f32,
    #[serde(rename = "Memo")]
    pub memo: String,
}