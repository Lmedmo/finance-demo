import type { Deposit, Expense, Transaction, Transactions, Transfer } from '$lib/models';
import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

/* Stores ----------------------------------------------------------------------------------------------- */
export const TransactionsStore: Writable<Transactions>  = writable()

/* Read ----------------------------------------------------------------------------------------------- */
export async function getTransactions(){
    const resp: Transactions = await invoke('cmd_get_transactions');
    TransactionsStore.set(resp);
    console.log("DB Fetch: Transactions");
}

/* Create ----------------------------------------------------------------------------------------------- */
export async function addTransfer(value: Transfer){
    console.log("Attempting to Add Transfer: ", value);
    const resp = await invoke('cmd_add_transfer', { transfer: value });
    console.log("Add Transfer Status: ", resp);
}
export async function addDeposit(value: Deposit){
    console.log("Attempting to Add Deposit: ", value);
    const resp = await invoke('cmd_add_deposit', { deposit: value });
    console.log("Add Deposit Status: ", resp);
}
export async function addExpense(value: Expense){
    console.log("Attempting to Add Expense: ", value);
    const resp = await invoke('cmd_add_expense', { expense: value });
    console.log("Add Expense Status: ", resp);
}

/* Delete ----------------------------------------------------------------------------------------------- */
export async function deleteTransactions(selected: number[]){
    for(const id of selected){
        console.log("Deleting Transaction: ", id);
        const resp = await invoke("cmd_delete_transaction", { id: id } );
        console.log("Deletion Status: ", resp);
    }
}

/* Update ----------------------------------------------------------------------------------------------- */
export async function editTransaction(values: Transaction, transactionID: number) {
    console.log(`Updating Transacitons: ${transactionID}, value = `, values)
    const resp = await invoke('cmd_edit_transaction', {t: values, id: transactionID})
    console.log(`Transaction update status: `, resp)
}