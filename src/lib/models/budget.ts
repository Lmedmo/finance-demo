import { BudgetsStore } from "$lib";
import type { Category } from '$lib/models'

const date = new Date();
const month = date.toLocaleString('default', {month: 'long'})

export const emptyBudgetCat: BudgetCategory = {
    category_id: 0,
    budget_id: 0,
    calc_type: 0,
    spending_limit: 0.00
}
export const emptyBudget: Budget = {
    month: month, //'Month',
    year: date.getFullYear()+'', //'Year'
    categories: []
}

let budgstr: Budgets;
BudgetsStore.subscribe((value)=>{ budgstr = value });

export function getBudget(id: number){
    let budget = emptyBudget
    for (const budg of budgstr.records){
        if (budg.id == id){
            budget = budg
            break
        }
    }
    const resp = [budget]
    return resp
}

export function getCatsAsBcats(bid: number | undefined, cats: Category[]){
    const bcats: BudgetCategory[] = []
    if(typeof bid === 'undefined'){
        bid = 0
    }
    for (const cat of cats){
        const bcat: BudgetCategory = {
            id: cat.id,
            category_id: cat.id,
            name: cat.name,
            budget_id: bid, 
            calc_type: 1,
            spending_limit: 0.00,
            icon: cat.icon, 
            icon_color: cat.icon_color
        }
        bcats.push(bcat)
    }
    const resp = bcats
    return resp
}
export interface Budgets {
    records: Budget[];
    types: BudgetType[];
}
export interface Budget {
    [index: string]: string | number | undefined | BudgetCategory[];
    id?: number;
    month: string;
    year: string;
    categories: BudgetCategory[];
}
export interface BudgetCategory {
    [index: string]: string | number | undefined;
    id?: number;
    category_id: number;
    name?: string;
    budget_id: number;
    calc_type: number;
    spending_limit: number;
    icon?: string;
    icon_color?: string;
    total?: number;
}
export interface NewBudgetRec {
    [index: string]: string | number | undefined;
    month: string;
    year: string;
}

export interface NewBudgetCategory {
    [index: string]: string | number | undefined;
    category_id: number;
    budget_id: number;
    calc_type: number;
    spending_limit: number;
}

export interface BudgetType {
    [index: string]: string | number | undefined;
    id: number;
    name: string;
}