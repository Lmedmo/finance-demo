#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod db;
mod commands;
mod models;
mod utilities;
mod controllers;

fn main() {
  tauri::Builder::default()
    .setup(|_app| {
      db::init();
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      // Create
      commands::cmd_add_transfer,
      commands::cmd_add_deposit,
      commands::cmd_add_expense,
      commands::cmd_add_credit_card,
      commands::cmd_add_mobile_account,
      commands::cmd_add_checking_account,
      commands::cmd_add_savings_account,
      commands::cmd_add_investment_account,
      commands::cmd_add_category,
      commands::cmd_add_user,
      commands::cmd_add_merchant,
      commands::cmd_add_depositor,
      commands::cmd_add_budget,
      commands::cmd_add_category_to_budget,
      // Read
      commands::cmd_get_transactions,
      commands::cmd_get_budgets,
      commands::cmd_get_accounts,
      commands::cmd_get_categories,
      commands::cmd_get_merchants,
      commands::cmd_get_depositors,
      commands::cmd_get_users,
      // Update
      commands::cmd_edit_account,
      commands::cmd_edit_transaction,
      commands::cmd_edit_depositor,
      commands::cmd_edit_budget_categories,
      commands::cmd_edit_user,
      commands::cmd_edit_category,
      commands::cmd_edit_merchant,
      // Delete
      commands::cmd_delete_transaction,
      commands::cmd_delete_account,
      commands::cmd_delete_custom_category,
      // Others
      commands::import_statement,
      commands::cmd_get_appstate,
      commands::cmd_set_appstate
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}