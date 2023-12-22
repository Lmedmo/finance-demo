mod accounts;
mod transactions;
mod categories;
mod merchants;
mod depositors;
mod budgets;
mod users;
mod memos;

pub use accounts::*;
pub use transactions::*;
pub use categories::*;
pub use merchants::*;
pub use users::*;
pub use depositors::*;
pub use budgets::*;
pub use memos::*;

use crate::{models::App, db::get_url};
use sqlx::sqlite::SqlitePool;

pub async fn get_appstate() -> Result<App, String>{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");

    let sql = "SELECT * FROM app";
    let query = sqlx::query_as::<_, App>(sql);
    let result = query.fetch_one(&pool).await.expect("unable to get app-state");

    pool.close().await;

    Ok(result)
}

pub async fn set_appstate(conf: App){
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let id = conf.id;
    let sql = "UPDATE app SET 
                   theme = $1,
                   font = $2
               WHERE id = $3;";
    
    let _result = sqlx::query(&sql)
        .bind(conf.theme)
        .bind(conf.font)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to save app state");
    pool.close().await;
}