use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
//use crate::models::*;
use crate::db::get_url;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Depositor {
    pub id: i32,
    pub name: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Depositors {
    records: Vec<Depositor>,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewDepositor {
    pub name: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct DepositorAlias {
    pub id: i32,
    pub depositor_id: i32,
    pub alias_name: String,
}

pub async fn get_depositors() -> Result<Depositors, String>{
    let db_url = get_url();

    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");

    let sql = "SELECT * FROM depositors";

    let query = sqlx::query_as::<_, Depositor>(sql);
    
    let depositors = query.fetch_all(&pool).await.expect("unable to list depositors");

    pool.close().await;

    let resp = Depositors { records: depositors };

    Ok(resp)
}
pub async fn get_depositor_name(id: i32) -> Result<String, String>{
    let mut sql = "SELECT * FROM depositors".to_owned();
    let filter_val = format!(r#" WHERE id = {id}"#);
    sql.push_str(&filter_val);
    let query = sqlx::query_as::<_, Depositor>(&sql);


    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let depositor = query.fetch_one(&pool).await.expect("unable to list depositors");
    pool.close().await;

    let resp = depositor.name;

    Ok(resp)
}

pub async fn add_depositor(d: NewDepositor)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO depositors 
                    (name) 
                VALUES ($1)";
    let result = sqlx::query(&sql)
        .bind(d.name)
        .execute(&pool)
        .await
        .expect("failed to add despositor");
    pool.close().await;
    let resp = format!("add_depositor: {:?}", result);
    return resp
}
pub async fn add_depositor_alias(d: DepositorAlias){
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO depositor_aliases 
                    (depositor_id, alias_name) 
                VALUES ($1, $2)";
    let _result = sqlx::query(&sql)
        .bind(d.depositor_id)
        .bind(d.alias_name)
        .execute(&pool)
        .await
        .expect("failed to add despositor alias");
    pool.close().await;
}

pub async fn edit_depositor(d: NewDepositor, id: i32)->String{
    println!("edit_depositor called: d = {:?}", d);
    println!("id = {:?}", id);
    
    let sql = "UPDATE depositors SET 
                   name = $1
               WHERE id = $2;";

    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let result = sqlx::query(&sql)
        .bind(d.name)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to update depositor");
    pool.close().await;
    let resp = format!("{:?}", result);
    return resp
}