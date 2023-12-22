use sqlx::sqlite::SqlitePool;
use crate::controllers::*;
use crate::db::get_url;

pub async fn check_merchant_category(cat_id: i32, merch_id: i32){
    let mut included = false;
    let mcats = get_merchant_categories(true, merch_id, cat_id).await.unwrap();
    if mcats.len() > 0 {
        included = true;
    } 

    if !included {
        let new_mcat = NewMerchantCategory {category_id: cat_id, merchant_id: merch_id};
        add_merchant_category(new_mcat).await;
    }
}
pub async fn check_depositor_aliases(desc: &String, depos_id: i32){
    let mut included = false;
    let mut sql = "SELECT * FROM depositor_aliases".to_owned();
    let filter_val = format!(" WHERE depositor_id = {}", depos_id);
    sql.push_str(&filter_val);
    let query = sqlx::query_as::<_, DepositorAlias>(&sql);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let aliases = query.fetch_all(&pool).await.expect("failed to check depositor aliases");
    pool.close().await;

    if aliases.len() > 0 {
        for alias in aliases {
            let keyword = alias.alias_name;
            if desc.contains(&keyword) {
                included = true;
                break;
            }
        }
    }

    if !included {
        let new_alias = DepositorAlias {id: 0, depositor_id: depos_id, alias_name: desc.to_owned()};
        add_depositor_alias(new_alias).await;
    }
}
pub async fn check_payment_transfer_aliases(desc: &String, acct_id: i32, tf_acct_id: i32, tf_acct_name: &String){
    let mut included = false;
    let mut sql = "SELECT * FROM payment_transfer_aliases".to_owned();
    let filter_val = format!(r#" WHERE account_id = {acct_id} AND to_from_account = {tf_acct_id}"#);
    sql.push_str(&filter_val);
    let query = sqlx::query_as::<_, PaymentTransferAlias>(&sql);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let aliases = query.fetch_all(&pool).await.expect("failed to check payment/transfer aliases");
    pool.close().await;

    if aliases.len() > 0 {
        for alias in aliases {
            let keyword = alias.alias_name;
            if desc.contains(&keyword) {
                included = true;
                break;
            }
        }
    }

    if !included {
        let alias = &desc[0..15];
        let new_alias = PaymentTransferAlias {id: 0, account_id: acct_id, to_from_account: tf_acct_id, alias_name: alias.to_string()};
        add_payment_transfer_alias(new_alias, tf_acct_name).await;
    }
}