use crate::models::*;
use crate::utilities::*;
use crate::controllers::*;

use tauri::command;

/* ------------------------------------ Create Functions --------------------------------------------- */
#[command]
pub async fn cmd_add_transfer(transfer: NewTransfer)->String{
    let resp = add_transfer(transfer).await;
    return resp
}

#[command]
pub async fn cmd_add_deposit(deposit: NewDeposit)->String{
    let resp = add_deposit(deposit).await;
    return resp
}

#[command]
pub async fn cmd_add_expense(expense: NewExpense)->String{
    let resp = add_expense(expense).await;
    return resp
}

#[command]
pub async fn cmd_add_user(user: NewUser)->String{
    let resp = add_user(user).await;
    return resp
}

#[command]
pub async fn cmd_add_credit_card(credit: NewCreditCard)->String{
    let resp = add_credit_card(credit).await;
    return resp
}

#[command]
pub async fn cmd_add_mobile_account(mobile: NewMobileAccount)->String{
    let resp = add_mobile_account(mobile).await;
    return resp
}

#[command]
pub async fn cmd_add_checking_account(checking: NewCheckingAccount)->String{
    let resp = add_checking_account(checking).await;
    return resp
}

#[command]
pub async fn cmd_add_savings_account(savings: NewSavingsAccount)->String{
    let resp = add_savings_account(savings).await;
    return resp
}

#[command]
pub async fn cmd_add_investment_account(invest: NewInvestmentAccount)->String{
    let resp = add_investment_account(invest).await;
    return resp
}

#[command]
pub async fn cmd_add_category(category: NewCategory)->String{
    let resp = add_category(category).await;
    return resp
}

#[command]
pub async fn cmd_add_merchant(merchant: NewMerchant)->String{
    let resp = add_merchant(merchant).await;
    return resp
}

#[command]
pub async fn cmd_add_depositor(depositor: NewDepositor)->String{
    let resp = add_depositor(depositor).await;
    return resp
}

#[command]
pub async fn cmd_add_budget(budget: NewBudget)->DBresp{
    let resp = add_budget(budget).await;
    //let resp = add_budget(budget).await.unwrap();
    return resp
    //Ok(resp)
}

#[command]
pub async fn cmd_add_category_to_budget(bcat: NewBudgetCategory)->String{
    let resp = add_category_to_budget(bcat).await;
    return resp
}

/* ----------------------------------------- Read Functions ------------------------------------------ */
#[command]
pub async fn cmd_get_users() -> Result<Users, String>{
    let result: Users = get_users().await.unwrap();
    Ok(result)
}

#[command]
pub async fn cmd_get_accounts() -> Result<Accounts, String>{
    let result: Accounts = get_accounts().await.unwrap();
    Ok(result)
}

#[command]
pub async fn cmd_get_categories() -> Result<Categories, String>{
    let result: Categories = get_categories().await.unwrap();
    Ok(result)
}

#[command]
pub async fn cmd_get_merchants() -> Result<Merchants, String>{
    let result: Merchants = get_merchants().await.unwrap();
    Ok(result)
}

#[command]
pub async fn cmd_get_depositors() -> Result<Depositors, String>{
    let result: Depositors = get_depositors().await.unwrap();
    Ok(result)
}

#[command]
pub async fn cmd_get_transactions() -> Result<Transactions, String>{
    let result: Transactions = get_transactions().await.unwrap();
    Ok(result)
}

#[command]
pub async fn cmd_get_budgets() -> Result<Budgets, String>{
    let result: Budgets = get_budgets().await.unwrap();
    Ok(result)
}

/* --------------------------------------- Update Functions ------------------------------------------ */
#[command]
pub async fn cmd_edit_account(a: NewAccount, id: i32)->String{
    let resp = edit_account(a, id).await;
    return resp
}
#[command]
pub async fn cmd_edit_user(u: NewUser, id: i32)->String{
    let resp = edit_user(u, id).await;
    return resp
}
#[command]
pub async fn cmd_edit_depositor(d: NewDepositor, id: i32)->String{
    let resp = edit_depositor(d, id).await;
    return resp
}
#[command]
pub async fn cmd_edit_category(c: NewCategory, id: i32)->String{
    let resp = edit_category(c, id).await;
    return resp
}
#[command]
pub async fn cmd_edit_merchant(m: NewMerchant, id: i32)->String{
    let resp = edit_merchant(m, id).await;
    return resp
}
#[command]
pub async fn cmd_edit_budget_categories(bcats: Vec<NewBudgetCategory>, id: i32)->Vec<String>{
    let resp = edit_budget_categories(bcats, id).await;
    return resp
}
#[command]
pub async fn cmd_edit_transaction(t: NewTransaction, id: i32)->String{
    let acct_id = t.account_id;
    let cat_id = t.category_id;
    let merch_id = t.merchant_id;
    let depos_id = t.depositor_id;
    let to_from_id = t.to_from_account;
    let tf_acct_name = t.to_from_acct_name.clone();
    let descr = t.description.clone();

    let mut checked = false;

    if cat_id.is_some() && merch_id.is_some() {
        let cid = cat_id.unwrap();
        let mid = merch_id.unwrap();
        check_merchant_category(cid, mid).await;
        checked = true;
    }

    if depos_id.is_some() && descr.is_some() {
        let did = depos_id.unwrap();
        let desc = &descr.clone().unwrap();
        check_depositor_aliases(&desc, did).await;
        checked = true;
    }

    if to_from_id.is_some() && tf_acct_name.is_some() {
        let tfid = to_from_id.unwrap();
        let tf_name = &tf_acct_name.clone().unwrap();
        let desc = &descr.clone().unwrap();
        check_payment_transfer_aliases(&desc, acct_id, tfid, &tf_name).await;
        checked = true;
    }

    let mut resp = "".to_string();

    if checked {
        resp = edit_transaction(t, id).await;
    }
    
    return resp
}

/* --------------------------------------- Delete Functions ------------------------------------------ */
#[command]
pub async fn cmd_delete_transaction(id: i32)->String{
    let resp = delete_transaction(id).await;
    return resp
}
#[command]
pub async fn cmd_delete_account(id: i32)->String{
    let resp = delete_account(id).await;
    return resp
}
#[command]
pub async fn cmd_delete_custom_category(id: i32)->String{
    let resp = delete_custom_category(id).await;
    return resp
}

/* --------------------------------------- Other Functions ------------------------------------------ */
#[command]
pub async fn import_statement(acct: i32, csvpath: String) {
    let acctrec: AccountRec = get_account_info(acct).await.expect("error retreiving account info");
    if acctrec.bank_id==2 {
        if let Err(e) = import_apple(acctrec, &csvpath){
            eprintln!("{}", e);
        }
    } else if acctrec.bank_id==4 {
        if let Err(e) = import_chase(acctrec, &csvpath){
            eprintln!("{}", e);
        }
    } else {
        println!("invalid format or no handler: {:?}", acctrec)
    }
}

#[command]
pub async fn cmd_get_appstate()-> Result<App, String> {
    let result: App = get_appstate().await.unwrap();
    Ok(result)
}

#[command]
pub async fn cmd_set_appstate(conf: App){
    set_appstate(conf).await;
}