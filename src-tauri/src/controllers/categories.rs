use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

use crate::db::get_url;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewCategory {
    pub name: String,
    pub icon: String,
    pub icon_color: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub icon: String,
    pub icon_color: String,
    pub type_id: i32,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Categories {
    records: Vec<Category>,
}

pub async fn get_categories() -> Result<Categories, String>{
    let sql = "SELECT * FROM categories";
    let query = sqlx::query_as::<_, Category>(sql);

    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let categories = query.fetch_all(&pool).await.expect("unable to list categories");
    pool.close().await;

    let resp = Categories { records: categories };

    Ok(resp)
}
pub async fn get_category_name(id: i32) -> Result<String, String>{
    let mut sql = "SELECT * FROM categories".to_owned();
    let filter_val = format!(r#" WHERE id = {id}"#);
    sql.push_str(&filter_val);
    let query = sqlx::query_as::<_, Category>(&sql);

    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let category = query.fetch_one(&pool).await.expect("unable to list depositors");
    pool.close().await;

    let resp = category.name;

    Ok(resp)
}

pub async fn add_category(c: NewCategory)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO categories 
                    (name, icon, icon_color, type_id) 
                VALUES ($1, $2, $3, $4)";
    let result = sqlx::query(&sql)
        .bind(c.name)
        .bind(c.icon)
        .bind(c.icon_color)
        .bind(1)
        .execute(&pool)
        .await
        .expect("failed to add category");
    pool.close().await;
    let resp = format!("add_category: {:?}", result);
    return resp
}
pub async fn edit_category(c: NewCategory, id: i32)->String{
    let sql = "UPDATE categories SET 
                   name = $1,
                   icon = $2,
                   icon_color = $3
               WHERE id = $4;";
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let result = sqlx::query(&sql)
        .bind(c.name)
        .bind(c.icon)
        .bind(c.icon_color)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to update category");
    pool.close().await;
    let resp = format!("edit_category: {:?}", result);
    return resp
}
pub async fn delete_custom_category(id: i32)->String{
    let sql = "DELETE FROM categories WHERE id = $1 AND type_id = 1;";
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let result = sqlx::query(&sql)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to delete category");
    pool.close().await;
    let resp = format!("delete_category: {:?}", result);
    return resp
}