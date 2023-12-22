use futures::executor;
use csv;
use sqlx::sqlite::SqlitePool;
use crate::models::*;
use crate::db::get_url;
use crate::controllers::*;

fn derive_transactions(ts: Vec<CSVRecord>, a: AccountRec){
    for t in ts {
        let t_type = derive_transaction_type(a.type_id, t.amount); // Returns either 2 (Deposit) or 3 (Withdrawal)
        let memo = executor::block_on(derive_memo(t_type.id, a.type_id, &t.description));
        let amt = t.amount.abs();
        let acct_name: String = a.name.clone();
        if memo.id == 8 || memo.id == 9 {
            let to_acct = executor::block_on(derive_to_account(&t.description, a.id));
            let transfer = NewTransfer {
                type_id: t_type.id,
                type_name: t_type.name,
                memo_id: memo.id,
                memo_name: memo.name,
                account_id: a.id,
                account_name: acct_name,
                to_from_account: to_acct.id,
                to_from_acct_name: to_acct.name,
                date: t.date,
                description: Some(t.description),
                amount: amt,
            };
            executor::block_on(add_transfer(transfer));
        } else if t_type.id == 2 {
            let depos = executor::block_on(derive_depositor(&t.description));
            let deposit = NewDeposit {
                type_id: t_type.id,
                type_name: t_type.name,
                memo_id: memo.id,
                memo_name: memo.name,
                account_id: a.id,
                account_name: acct_name,
                date: t.date,
                depositor_id: depos.id,
                depositor_name: depos.name,
                description: Some(t.description),
                amount: amt,
            };
            executor::block_on(add_deposit(deposit));
        } else if t_type.id == 3{
            let merch = executor::block_on(derive_merchant(&t.description));
            let cat = executor::block_on(derive_category(merch.id));
            let expense = NewExpense {
                type_id: t_type.id,
                type_name: t_type.name,
                memo_id: memo.id,
                memo_name: memo.name,
                account_id: a.id,
                account_name: acct_name,
                date: t.date,
                merchant_id: merch.id,
                merchant_name: merch.name,
                description: Some(t.description),
                category_id: cat.id,
                category_name: cat.name,
                amount: amt,
            };
            executor::block_on(add_expense(expense));
        } else {
            println!("Failed to derive transaction attributes...");
            println!("csv record values: {:?}", t);
            println!("account values: {:?}", a)
        }
    } 
}

pub fn import_apple(acct: AccountRec, path: &str) -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut trimmed_data: Vec<CSVRecord> = Vec::new();

    for result in reader.deserialize() {
        let record: AppleCardRec = result?;

        let t_descrip = record.description;
        let t_date = record.transaction_date;
        let t_amt = record.amount;

        let trimmed_rec = CSVRecord {
            date: t_date,
            description: t_descrip,
            amount: t_amt,
        };
        trimmed_data.push(trimmed_rec);
    }
    derive_transactions(trimmed_data, acct);
    Ok(())
}
pub fn import_chase(acct: AccountRec, path: &str) -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut trimmed_data: Vec<CSVRecord> = Vec::new();

    for result in reader.deserialize() {
        let record: ChaseRec = result?;

        let t_descrip = record.description;
        let t_date = record.transaction_date;
        let t_amt = record.amount * -1.0;

        let trimmed_rec = CSVRecord {
            date: t_date,
            description: t_descrip,
            amount: t_amt,
        };
        trimmed_data.push(trimmed_rec);
    }
    derive_transactions(trimmed_data, acct);
    Ok(())
}

fn derive_transaction_type(acct_type: i32, amt: f32)->CheckKVP{
    /* Context Info >
       Transaction Types
        1 ('Unrecognized'), 2 ('Deposit'), 3 ('Withdrawal')
       Account Types 
        1 ('Credit Account'),  2 ('Mobile Account'),     3 ('Checking Account'),
        4 ('Savings Account'), 5 ('Investment Account'), 6 ('Other') 
    */
    let type_id;
    let type_name;
    
    if acct_type == 1 {
        if amt > 0.0 {
            type_id = 3;
            type_name = "Withdrawal".to_string();
        } else {
            type_id = 2;
            type_name = "Deposit".to_string();
        }
    } else if acct_type == 3 {
        if amt > 0.0 {
            type_id = 2;
            type_name = "Deposit".to_string();
        } else {
            type_id = 3;
            type_name = "Withdrawal".to_string();
        }
    } else { 
        if amt > 0.0 {
            type_id = 2;
            type_name = "Deposit".to_string();
        } else {
            type_id = 3;
            type_name = "Withdrawal".to_string();
        }
    }
    let resp = CheckKVP { id: type_id, name: type_name};
    return resp;
}
async fn derive_memo(t_type: i32, acct_type: i32, desc: &String)->CheckKVP {
    // Memos
    /* 1 ('Unrecognized'), 2 ('Purchase'), 3 ('Fee'),      4 ('Cash Withdrawal'), 5 ('Income')
       6 ('Cash Deposit'), 7 ('Refund'),   8 ('Transfer'), 9 ('Payment'),        10 ('Verification'); */
    let mut memo_id = 1;
    let mut memo_name: String = "Unrecognized".to_string();

    if t_type == 1 {
        memo_id = 1;
        memo_name = "Unrecognized".to_string();
    } else if t_type == 2 && acct_type == 1 { // Credit Card Deposit = Payment
        memo_id = 9;
        memo_name = "Payment".to_string();
    } else if t_type == 3 && acct_type == 1 { // Credit Card Withdrawal = Purchase
        memo_id = 2;
        memo_name = "Purchase".to_string();
    } else {
        let mut sql = "SELECT * FROM memo_aliases".to_owned();
        let filter_val = format!(r#" WHERE type_id = {t_type}"#);
        sql.push_str(&filter_val);

        let db_url = get_url();
        let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
        let query = sqlx::query_as::<_, MemoAlias>(&sql);
        let aliases = query.fetch_all(&pool).await.expect("unable to list memo aliases");
        pool.close().await;

        for alias in aliases {
            let keyword = alias.alias_name;
            if desc.contains(&keyword) {
                memo_id = alias.memo_id;
                memo_name = get_memo_name(memo_id).await.unwrap();
                break;
            }
        }
    }
    let memo = CheckKVP {id: memo_id, name: memo_name};
    return memo;
}
async fn derive_to_account(desc: &String, acct_id: i32)->CheckKVP {
    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    
    let mut sql = "SELECT * FROM payment_transfer_aliases".to_owned();
    let filter_val = format!(r#" WHERE account_id = {acct_id}"#);
    sql.push_str(&filter_val);

    let query = sqlx::query_as::<_, PaymentTransferAlias>(&sql);
    let result = query.fetch_all(&pool).await.expect("unable to list payment_transfer_aliases");
    pool.close().await;

    let mut to_acct_id = 1;
    let mut to_acct_name: String = "Unrecognized".to_string();

    if result.len() > 0 {
        for val in result {
            let keyword = val.alias_name;
            if desc.contains(&keyword) {
                to_acct_id = val.to_from_account;
                let to_acct_full = get_account_info(to_acct_id).await.unwrap();
                to_acct_name = to_acct_full.name;
                break;
            }
        }
    }
    let to_acct = CheckKVP {id: to_acct_id, name: to_acct_name};
    return to_acct
}
async fn derive_merchant(desc: &String)->CheckKVP{
    let sql = "SELECT * FROM merchant_aliases";
    let query = sqlx::query_as::<_, MerchantAlias>(&sql);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let aliases = query.fetch_all(&pool).await.expect("unable to list merchant aliases");
    pool.close().await;

    let mut merch_id = 1;
    let mut merch_name = "Unrecognized".to_string();

    for alias in aliases {
        let keyword = alias.alias_name;
        if desc.contains(&keyword) {
            merch_id = alias.merchant_id;
            merch_name = get_merchant_name(merch_id).await.unwrap();
            break;
        }
    }
    let merch = CheckKVP {id: merch_id, name: merch_name};
    return merch
}
async fn derive_depositor(desc: &String)->CheckKVP {
    let mut depos_id = 1;
    let mut depos_name = "Unrecognized".to_string();

    let sql = "SELECT * FROM depositor_aliases";
    let query = sqlx::query_as::<_, DepositorAlias>(&sql);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let result = query.fetch_all(&pool).await.expect("unable to list depositor aliases");
    pool.close().await;

    for val in result {
        let keyword = val.alias_name;
        if desc.contains(&keyword) {
            depos_id = val.depositor_id;
            depos_name = get_depositor_name(depos_id).await.unwrap();
            break;
        }
    }
    let depos = CheckKVP {id: depos_id, name: depos_name};
    return depos
}
async fn derive_category(merchant_id: i32)->CheckKVP{
    let mut cat_id = 1;
    let mut cat_name = "Unrecognized".to_string();

    let mut sql = "SELECT * FROM merchant_categories".to_owned();
    let filter_val = format!(r#" WHERE merchant_id = {merchant_id}"#);
    sql.push_str(&filter_val);
    let query = sqlx::query_as::<_, MerchantCategory>(&sql);

    let db_url = get_url();
    let pool = SqlitePool::connect(&db_url).await.expect("unable to connect");
    let mcats = query.fetch_all(&pool).await.expect("unable to list merchant-categories");
    pool.close().await;
    
    if mcats.len() > 0 {
        cat_id = mcats[0].category_id;
        cat_name = get_category_name(cat_id).await.unwrap();
    }

    let cat = CheckKVP {id: cat_id, name: cat_name};
    return cat
}
