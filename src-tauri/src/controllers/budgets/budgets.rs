use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use crate::db::get_url;
use crate::models::DBresp;
use crate::utilities::*;

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct NewBudget { // For DB
    pub month: String,
    pub year: String,
}
#[derive(Debug,Clone,Copy,FromRow,Deserialize,Serialize)]
pub struct NewBudgetCategory { // For DB
    pub category_id: i32,
    pub budget_id: i32,
    pub calc_type: i32,
    pub spending_limit: f32,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct BudgetRec { // From DB
    pub id: i32,
    pub month: String,
    pub year: String
}
#[derive(Debug,Clone,Copy,FromRow,Deserialize,Serialize)]
pub struct BudgetCategoryRec { // From DB
    pub id: i32,
    pub category_id: i32,
    pub budget_id: i32,
    pub calc_type: i32,
    pub spending_limit: f32,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct BudgetCategory { // Total attached
    pub id: i32,
    pub category_id: i32,
    pub budget_id: i32,
    pub calc_type: i32,
    pub spending_limit: f32,
    pub total: f64,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Budget { // BudgetRec Info attached to array of categories
    pub id: i32,
    pub month: String,
    pub year: String,
    categories: Vec<BudgetCategory>
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct BudgetType {
    pub id: i32,
    pub name: String,
}
#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Budgets {
    records: Vec<Budget>,
    types: Vec<BudgetType>,
}

pub async fn get_budgets() -> Result<Budgets, String>{
    let budgetssql = "SELECT * FROM budgets";
    let budget_query = sqlx::query_as::<_, BudgetRec>(&budgetssql);
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let budgetrecs = budget_query.fetch_all(&pool).await.expect("unable to list budgets");
    let types = get_budget_calc_types().await.unwrap();

    let mut budgets: Vec<Budget> = Vec::new();

    for rec in budgetrecs {
        let bcats = get_budget_categories(true, rec.id).await.unwrap();
        
        let mut categories: Vec<BudgetCategory> = Vec::new();

        for bcat in bcats {
            let cat_total = get_category_total(bcat.category_id).await.unwrap();
            let cat =  BudgetCategory { 
                id: bcat.id, 
                category_id: bcat.category_id,
                budget_id: bcat.budget_id,
                calc_type: bcat.calc_type,
                spending_limit: bcat.spending_limit,
                total: cat_total 
            };
            categories.push(cat);
        }
        let budget = Budget { id: rec.id, month: rec.month, year: rec.year, categories };
        budgets.push(budget)
    }

    pool.close().await;

    let resp = Budgets { records: budgets, types };

    Ok(resp)
}
pub async fn get_budget_categories(filtered: bool, _id: i32)->Result<Vec<BudgetCategoryRec>, String> {
    let mut sql = "SELECT * FROM budget_categories".to_owned();

    if filtered {
        let filter = format!(" WHERE budget_id = {}", _id);
        sql.push_str(&filter);
    }

    let query = sqlx::query_as::<_, BudgetCategoryRec>(&sql);
    let pool = SqlitePool::connect(&get_url()).await.expect("unable to connect");
    let result = query.fetch_all(&pool).await.expect("unable to list budget categories");
    pool.close().await;
    Ok(result)
}
pub async fn get_budget_calc_types()->Result<Vec<BudgetType>, String>{
    let btypesql = "SELECT * FROM budget_calc_types";
    let btype_query = sqlx::query_as::<_, BudgetType>(&btypesql);
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let types = btype_query.fetch_all(&pool).await.expect("unable to list budget_calc_types");
    pool.close().await;
    Ok(types)
}

pub async fn add_budget(b: NewBudget)->DBresp{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO budgets 
                    (month, year) 
                VALUES ($1, $2)";
    let result = sqlx::query(&sql)
        .bind(b.month)
        .bind(b.year)
        .execute(&pool)
        .await
        .expect("failed to add budget");
    pool.close().await;
    let resp = DBresp {changes: result.rows_affected(), last_insert_rowid: result.last_insert_rowid()};
    return resp
}
pub async fn add_category_to_budget(b: NewBudgetCategory)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "INSERT INTO budget_categories 
                    (category_id, budget_id, calc_type, spending_limit) 
                VALUES ($1, $2, $3, $4)";
    let result = sqlx::query(&sql)
        .bind(b.category_id)
        .bind(b.budget_id)
        .bind(b.calc_type)
        .bind(b.spending_limit)
        .execute(&pool)
        .await
        .expect("failed to add category to budget");
    pool.close().await;
    let resp = format!("add_category_to_budget: {:?}", result);
    return resp
}
pub async fn delete_category_from_budget(b: BudgetCategoryRec)->String{
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let sql = "DELETE FROM budget_categories WHERE 
                    category_id = $1 AND budget_id = $2;";
    let result = sqlx::query(&sql)
        .bind(b.category_id)
        .bind(b.budget_id)
        .execute(&pool)
        .await
        .expect("failed to delete category to budget");
    pool.close().await;
    let resp = format!("delete_category_from_budget: {:?}", result);
    return resp
}
pub async fn edit_budget_categories(bcats: Vec<NewBudgetCategory>, id: i32)->Vec<String>{    
    println!("bcats: {:?}", bcats);
    println!("id: {}", id);
    let current_bcats = get_budget_categories(true, id).await.unwrap();

    let mut msgs: Vec<String> = Vec::new();

    let mut addmsgs = add_new(&current_bcats, &bcats).await;
    let mut delmsgs = delete_removed(&current_bcats, &bcats).await;

    msgs.append(&mut addmsgs);
    msgs.append(&mut delmsgs);

    return msgs
}
async fn add_new(cur: &Vec<BudgetCategoryRec>, upd: &Vec<NewBudgetCategory>)-> Vec<String>{
    let mut msgs: Vec<String> = Vec::new();
    let upd_bcats: Vec<NewBudgetCategory> = upd.to_owned();

    for bcat in upd_bcats {
        let bcat_bid = bcat.budget_id;
        let bcat_cid = bcat.category_id;
        let mut exists = false;
        let cur_bcats: Vec<BudgetCategoryRec> = cur.to_owned();
        for cur_bcat in cur_bcats {
            if cur_bcat.budget_id == bcat_bid && cur_bcat.category_id == bcat_cid {
                exists = true;
                break
            }
        }
        if !exists {
            let resp = add_category_to_budget(bcat).await;
            msgs.push(resp);
        }
    }
    return msgs
}
async fn delete_removed(cur: &Vec<BudgetCategoryRec>, upd: &Vec<NewBudgetCategory>)->Vec<String>{
    let mut msgs: Vec<String> = Vec::new();
    let cur_bcats = cur.to_owned();
    
    for cur_bcat in cur_bcats {
        let cur_bcat_bid = cur_bcat.budget_id;
        let cur_bcat_cid = cur_bcat.category_id;
        let mut exists = false;
        let upd_bcats = upd.to_owned();
        for bcat in upd_bcats {
            if bcat.budget_id == cur_bcat_bid && bcat.category_id == cur_bcat_cid {
                exists = true;
                break
            }
        }
        if !exists {
            let resp = delete_category_from_budget(cur_bcat).await;
            msgs.push(resp);
        }
    }
    return msgs
}