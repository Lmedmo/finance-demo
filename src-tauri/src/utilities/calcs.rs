use sqlx::sqlite::SqlitePool;
use crate::db::get_url;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use round::round;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Total{
    pub amt: f64,
}

pub async fn get_account_bal(id: i32) -> Result<f64, String>{
    let mut withdrawal_sql = "SELECT ROUND(SUM(amount), 2) AS amt FROM transactions WHERE type_id = 3 AND".to_owned();
    let mut deposit_sql = "SELECT ROUND(SUM(amount), 2) AS amt FROM transactions WHERE type_id = 2 AND".to_owned();

    let filter_val = format!(" account_id = {};", id);
    if id != 0 {
       withdrawal_sql.push_str(&filter_val);
       deposit_sql.push_str(&filter_val);
    }

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");

    let w_query = sqlx::query_as::<_, Total>(&withdrawal_sql);
    let d_query = sqlx::query_as::<_, Total>(&deposit_sql);

    let w_result = w_query.fetch_one(&pool).await.expect("unable to get account balance");
    let d_result = d_query.fetch_one(&pool).await.expect("unable to get account balance");

    pool.close().await;

    let bal = d_result.amt - w_result.amt;
    let result = round(bal, 2);

    Ok(result)
}
pub async fn get_category_total(id: i32) -> Result<f64, String>{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");

    let mut category_sql = "SELECT ROUND(SUM(amount), 2) AS amt FROM transactions WHERE".to_owned();
    let filter_val = format!(" category_id = {};", id);
    category_sql.push_str(&filter_val);    

    let query = sqlx::query_as::<_, Total>(&category_sql);
    let total = query.fetch_one(&pool).await.expect("unable to get category total");

    pool.close().await;

    let result = round(total.amt, 2);

    Ok(result)
}

/* formula for compounding interest
    A = P(1+(r/n))^nt
    
    A -> Final amount
    P -> Initial/Principal amount
    r -> Interest Rate
    n -> Compounding Frequency (ie Times interest applied per period)
    t -> Periods elapsed */