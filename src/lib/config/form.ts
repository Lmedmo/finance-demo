import { writable, type Writable } from 'svelte/store';
import type { Account, Budget, BudgetCategory,  Category, Depositor, Merchant, Transaction, User } from '$lib/models';
import { emptyBudget, emptyBudgetCat, emptyCategory, emptyChecking, emptyCreditCard, emptyDeposit, emptyDepositor, 
    emptyExpense, emptyInvestment, emptyMerchant, emptyMobileAcct, emptySavings, emptyTransfer, emptyUser } from '$lib/models';
import type { Explorer } from './explorer';
import { setEmptyErrorMsg } from './errors';

/* Stores ---------------------------------------------------------------------------------------------------------------------- */
export const FormState: Writable<boolean> = writable();
export const FormConfig: Writable<FormConfigObj> = writable();

/* Types ----------------------------------------------------------------------------------------------------------------------- */
export interface FormConfigObj {
    formtype: FormType;
    rectype: RecordType;
    subjects: SelectedRecord[]; /*  Subjects   >> Holds a list of selected records that are subject to modification */
    collection: SelectedRecord; /*  Collection >> Holds the values (a 'fieldID:value' pair) collected from the form fields */
    explorer?: Explorer;        /*  Explorer   >> Holds the value of the ExplorerState at the time of Form initialization */
}
export interface Collection {
    [index: string]: string | number | undefined;
    id: number;
    name: string;
}
export type FormType = "Entry" | "Edit" | "Delete" | "Import" | "";

export type RecordType = 
/* Transactions */
    "Expense" | "Deposit" | "Transfer" | "Transaction" | "Transactions" | "Statement" |
/* Accounts */
    "Account" | 'Credit Card' | 'Mobile Account' | 'Checking Account' | 'Savings Account' | 'Investment Account' |
/* Budgets */
    "Budget" | "BudgetCategory" |
/* Others */
    "User" | "Category" | "Merchant" | "Depositor" | "";

export type SelectedRecord = Transaction | Account | Category | Budget | 
    BudgetCategory | Merchant | User | Depositor | Collection;

export type SelectorOption = {id: number; name: string};

/* Default values -------------------------------------------------------------------------------------------------------------- */
export const defaultCollection: Collection = {
    id: 0,
    name: "Default"
}
export const defaultFormConfig: FormConfigObj = {
    formtype: "", 
    rectype: "",
    subjects: [],
    collection: defaultCollection
}
export const emptySelectorOption: SelectorOption = { id: 0, name: ''}

/* Functions ------------------------------------------------------------------------------------------------------------------- */
let config: FormConfigObj;
FormConfig.subscribe((value)=>{config = value})

export function showForm() {
    console.log(`form controller >> [fn]: 'showForm'`)
    FormState.set(true);
}
export function closeForm() {
    console.log(`form controller >> [fn]: 'closeForm'`)
    config.formtype = "";
    config.rectype = "";
    config.subjects = [];
    config.collection = {id: 0, name: "Default"};
    config.explorer = undefined;
    FormConfig.set(config);
    console.log("FC after closeForm: ", config)
    FormState.set(false);
}
export function setCollection(rec: SelectedRecord){
    console.log(`form controller >> [fn]: 'setCollection'; [args]: 'rec' = `, rec)
    config.collection = rec
    FormConfig.set(config)
}
export function getEmpty(type: RecordType){
    console.log(`form controller >> [fn]: 'setEmpty'... [args]: 'type'...`, type)

    if (type === "Deposit"){
        return emptyDeposit
    } else if (type === "Expense"){
        return emptyExpense
    } else if (type === "Transfer"){
        return emptyTransfer
    } else if (type === "Credit Card"){
        return emptyCreditCard
    } else if (type === "Checking Account"){
        return emptyChecking
    } else if (type === "Mobile Account"){
        return emptyMobileAcct
    } else if (type === "Savings Account"){
        return emptySavings
    } else if (type === "Investment Account"){
        return emptyInvestment
    } else if (type === "Category"){
        return emptyCategory
    } else if (type === "Depositor"){
        return emptyDepositor
    } else if (type === "User"){
        return emptyUser
    } else if (type === "Budget"){
        return emptyBudget
    } else if (type === "BudgetCategory"){
        return emptyBudgetCat
    } else if (type === "Merchant"){
        return emptyMerchant
    } else {
        console.log(setEmptyErrorMsg)
        return defaultCollection
    }
}