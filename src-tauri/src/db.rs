use std::fs;
use futures::executor;
use std::path::Path;
use tauri::api::path::home_dir;
use sqlx::SqlitePool;

pub fn init() {
    if !db_exists() {
        create_db();
    }
    executor::block_on(migrate_schema());
}

fn create_db() {
    let db_path = get_url();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_exists() -> bool {
    let db_path = get_url();
    Path::new(&db_path).exists()
}

pub fn get_url() -> String {
    let home_dir = home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/finance-demo/finance-demo.sqlite"
}

async fn migrate_schema() {
    let db_path = get_url();
    let pool = SqlitePool::connect(&db_path).await.unwrap();
    let _result = sqlx::migrate!("./migrations").run(&pool).await;
    pool.close().await;
}