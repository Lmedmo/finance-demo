import type { Accounts, CheckingAccount, CreditCard, InvestmentAccount, MobileAccount, NewAccount, SavingsAccount } from '$lib/models';
import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

/* Stores ----------------------------------------------------------------------------------------------- */
export const AccountsStore: Writable<Accounts> = writable()

/* Read ----------------------------------------------------------------------------------------------- */
export async function getAccounts(){
    const resp: Accounts = await invoke('cmd_get_accounts');
    AccountsStore.set(resp);
    console.log("DB Fetch: Accounts... return: ", resp);
}

/* Create ----------------------------------------------------------------------------------------------- */
export async function addCreditCard(value: CreditCard){
    console.log("Attempting to Add Credit-Card: ", value);
    const resp = await invoke('cmd_add_credit_card', { credit: value });
    console.log("Add Credit-Card Status: ", resp);
}
export async function addMobileAccount(value: MobileAccount){
    console.log("Attempting to Add Mobile Account: ", value);
    const resp = await invoke('cmd_add_mobile_account', { mobile: value });
    console.log("Add Mobile-Account Status: ", resp);
}
export async function addCheckingAccount(value: CheckingAccount){
    console.log("Attempting to Add Checking Account: ", value);
    const resp = await invoke('cmd_add_checking_account', { checking: value });
    console.log("Add Checking Account Status: ", resp);
}
export async function addSavingsAccount(value: SavingsAccount){
    console.log("Attempting to Add Savings Account: ", value);
    const resp = await invoke('cmd_add_savings_account', { savings: value });
    console.log("Add Savings Account Status: ", resp);
}
export async function addInvestmentAccount(value: InvestmentAccount){
    console.log("Attempting to Add Investment Account: ", value);
    const resp = await invoke('cmd_add_investment_account', { invest: value });
    console.log("Add Investment Account Status: ", resp);
}

/* Delete ----------------------------------------------------------------------------------------------- */
export async function deleteAccount(id: number){
    console.log("Deleting Account: ", id)
    const resp = await invoke("cmd_delete_account", { id: id } );
    console.log("Deletion Status: ", resp);
}

/* Update ----------------------------------------------------------------------------------------------- */
export async function editAccount(values: NewAccount, acctID: number) {
    console.log(`Updating Account: ${acctID}, value = `, values)
    const resp = await invoke('cmd_edit_account', {a: values, id: acctID})
    console.log(`Account update status: `, resp)
}