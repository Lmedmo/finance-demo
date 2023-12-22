use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
//use crate::models::*;
use crate::db::get_url;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewUser {
    pub name: String,
    pub last_name: String,
    pub username: String,
    pub pin: i32,
    pub password: String,
    pub icon: String,
    pub icon_color: String,
    pub require_auth: bool,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name: String,
    pub username: String,
    pub pin: i32,
    pub password: String,
    pub icon: String,
    pub icon_color: String,
    pub require_auth: bool,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Users {
    pub records: Vec<User>,
}
pub async fn get_users() -> Result<Users, String>{
    let sql = "SELECT * FROM users";
    let query = sqlx::query_as::<_, User>(sql);

    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let result = query.fetch_all(&pool).await.expect("unable to list users");
    pool.close().await;

    let resp = Users { records: result };
    Ok(resp)
}

pub async fn add_user(u: NewUser)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO users 
                    (name, last_name, username, pin, password, icon, icon_color, require_auth) 
                VALUES ($1, $2, $3, $4, $5, $6, $7)";
    let result = sqlx::query(&sql)
        .bind(u.name)
        .bind(u.last_name)
        .bind(u.username)
        .bind(u.pin)
        .bind(u.password)
        .bind(u.icon)
        .bind(u.icon_color)
        .bind(u.require_auth)
        .execute(&pool)
        .await
        .expect("failed to add user");
    pool.close().await;
    let resp = format!("add_user: {:?}", result);
    return resp
}
pub async fn edit_user(u: NewUser, id: i32)->String{
    let sql = "UPDATE users SET 
                   name = $1,
                   last_name = $2,
                   username = $3,
                   pin = $4,
                   password = $5,
                   icon = $6,
                   icon_color = $7,
                   require_auth = $8
               WHERE id = $9;";
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let result = sqlx::query(&sql)
        .bind(u.name)
        .bind(u.last_name)
        .bind(u.username)
        .bind(u.pin)
        .bind(u.password)
        .bind(u.icon)
        .bind(u.icon_color)
        .bind(u.require_auth)
        .bind(id)
        .execute(&pool)
        .await
        .expect("failed to update user");
    pool.close().await;
    let resp = format!("edit_user: {:?}", result);
    return resp
}