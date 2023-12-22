import type { BudgetCategory, Budgets, NewBudgetCategory, NewBudgetRec } from '$lib/models';
import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';
import type { SqliteQueryResult } from '$lib';

/* Stores ----------------------------------------------------------------------------------------------- */
export const BudgetsStore: Writable<Budgets> = writable()

/* Read ----------------------------------------------------------------------------------------------- */
export async function getBudgets(){
    const resp: Budgets = await invoke('cmd_get_budgets');
    BudgetsStore.set(resp)
    console.log("DB Fetch: Budgets... Response: ", resp);
}

/* Create ----------------------------------------------------------------------------------------------- */
export async function addBudget(value: NewBudgetRec){
    console.log("Attempting to Add Budget: ", value);
    const resp: SqliteQueryResult = await invoke('cmd_add_budget', { budget: value });
    console.log("Add Budget Status: ", resp);
    return resp.last_insert_rowid
}
export async function addCategoryToBudget(value: NewBudgetCategory){
    console.log("Attempting to Add Budget-Category: ", value);
    const resp = await invoke('cmd_add_category_to_budget', { bcat: value });
    console.log("Add Budget-Category Status: ", resp);
}

/* Update ----------------------------------------------------------------------------------------------- */
export async function editBudgetCategories(values: BudgetCategory[], budgetcatID: number){
    console.log("Updating Budget-Categories: ", values);
    const resp = await invoke('cmd_edit_budget_categories', { bcats: values, id: budgetcatID });
    console.log("Budget-Categories update status: ", resp);
}