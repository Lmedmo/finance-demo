import { getAccount, getAccountType, getBudget, getCategory, getDepositor } from "$lib";
import type { Account, Transaction, Depositor, Merchant, Category, Budget, BudgetCategory } from "$lib/models";
import { FormConfig, showForm, getTable, closeForm } from "$lib/config";
import type { FormConfigObj, RecordType, Explorer } from "$lib/config";
import { missingLogicErrorMsg, editRecErrorMsg } from "$lib/config/errors";
import { editAccount, editTransaction, editDepositor, editMerchant, editCategory, editBudgetCategories } from "$lib/controllers";

let config: FormConfigObj;
FormConfig.subscribe((value)=>{ config = value });

export const editableTypes: RecordType[] = [
// Transactions
    'Expense', 
    'Transfer', 
    'Deposit',
// Accounts 
    'Account', 
    'Credit Card', 
    'Mobile Account', 
    'Checking Account', 
    'Savings Account', 
    'Investment Account',
// Others
    'Depositor',
    'Budget',
    'Category',
];
export function initEditForm(recordType: RecordType, explCtx: Explorer){
    console.log(`Action >> [fn]: 'initEditForm'... [args]: 'recordType' = `, recordType)
    console.log(`Action >> [fn]: 'initEditForm'... [args]: 'explCtx' = `, explCtx)

    let valid = false;

    for(const rectype of editableTypes){
        if (rectype === recordType){
            valid = true
            break
        }
    }
    if (valid){
        if (recordType === "Account"){
            config.rectype = getAccountType(explCtx.selected.type) as RecordType
            config.subjects = getAccount(explCtx.selected.id)
        } else if (recordType === "Budget"){
            config.rectype = recordType
            config.subjects = getBudget(explCtx.selected.id)
        } else if (recordType === "Category"){
            config.rectype = recordType
            config.subjects = getCategory(explCtx.selected.id)
        } else if (recordType === "Depositor"){
            config.rectype = recordType
            config.subjects = getDepositor(explCtx.selected.id)
        } else {
            config.rectype = recordType
        }
        config.formtype = "Edit"
        config.collection = config.subjects[0]
        config.explorer = explCtx
        FormConfig.set(config)
        showForm()
    } else {
        console.log("Type is not Editable")
    }
}

export async function editRec() {
    const rs = config.subjects;
    const rt = config.rectype;
    const rc = config.collection;

    console.log(`Action >> [fn]: 'editRec'... [collection]: `, rc)
    console.log(`Action >> [fn]: 'editRec'... [rectype]: `, rt)
    console.log(`Action >> [fn]: 'editRec'... [subjects]: `, rs)

    let valid = false

    for (const edittype of editableTypes){
        if (rt === edittype){
            valid = true
            break
        }
    }
    if (valid) {
        if (rt === "Account" || rt === "Checking Account" || rt === "Savings Account" || rt === "Mobile Account" || rt === "Investment Account" || rt === "Credit Card"){
            const account: Account = rc as Account;
            const sub = rs[0] as Account;
            const id = sub.id as number;
            await editAccount(account, id);
            await getTable("Transactions");
            await getTable("Accounts");
        } else if (rt === "Expense" || rt === "Deposit" || rt === "Transfer"){
            const transaction: Transaction = rc as Transaction;
            const sub = rs[0] as Transaction;
            const id = sub.id as number;
            await editTransaction(transaction, id);
            await getTable("Transactions");
            await getTable("Accounts");
        } else if (rt === "Depositor"){
            const depositor: Depositor = rc as Depositor;
            const sub = rs[0] as Depositor;
            const id = sub.id as number;
            await editDepositor(depositor, id);
            await getTable("Transactions");
            await getTable("Depositors");
        } else if (rt === "Merchant"){
            const merchant: Merchant = rc as Merchant;
            const sub = rs[0] as Merchant;
            const id = sub.id as number;
            await editMerchant(merchant, id);
            await getTable("Transactions");
            await getTable("Merchants");
        } else if (rt === "Category"){
            const category: Category = rc as Category;
            const sub = rs[0] as Category;
            const id = sub.id as number;
            await editCategory(category, id);
            await getTable("Transactions");
            await getTable("Categories");
        } else if (rt === "Budget"){
            const budget: Budget = rc as Budget;
            const bcats: BudgetCategory[] = budget.categories
            const id = budget.id as number;
            await editBudgetCategories(bcats, id);
            await getTable("Budgets");
        } else {
            console.log(missingLogicErrorMsg)
        }
    } else {
        console.log(editRecErrorMsg)
    }
    closeForm();
}