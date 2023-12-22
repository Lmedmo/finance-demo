import type { Merchant, Expense, Deposit, Transfer, MobileAccount, CreditCard, CheckingAccount, SavingsAccount, InvestmentAccount, Category, Depositor, User, Budget, BudgetCategory, NewBudgetRec, NewBudgetCategory } from "$lib";
import { getTable, type Explorer } from "$lib/config";
import { missingLogicErrorMsg, addRecErrorMsg } from "$lib/config/errors";
import { FormConfig, closeForm, getEmpty, showForm, type FormConfigObj, type RecordType } from "$lib/config/form";
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { addExpense, addDeposit, addTransfer, addMobileAccount, addCreditCard, addCheckingAccount, addSavingsAccount, addInvestmentAccount, addBudget, addCategoryToBudget, addCategory, addDepositor, addUser, addMerchant } from "$lib/controllers";

let config: FormConfigObj;
FormConfig.subscribe((value)=>{ config = value });

export function initAddForm(recordType: RecordType, explCtx: Explorer){
    const msg = "'recordType' = " + recordType + "... 'explCtx' = " + explCtx;
    console.log(`Action >> [fn]: 'initAddForm'... [args]: `, msg)

    let valid = false;
    for(const rectype of addableTypes){
        if (rectype === recordType){
            valid = true
            break
        }
    }
    if (valid){
        config.formtype = "Entry"
        config.rectype = recordType
        config.collection = getEmpty(recordType)
        config.explorer = explCtx
        FormConfig.set(config)
        showForm()
    } else {
        console.log("Type is not Addable")
    }
}
export const addableTypes: RecordType[] = [
// Transactions
    'Expense', 
    'Transfer', 
    'Deposit',
// Accounts
    'Credit Card', 
    'Mobile Account', 
    'Checking Account', 
    'Savings Account', 
    'Investment Account',
// Budgets
    'Budget',
// Others
    'Category',
    'Depositor',
    'User',
    'Merchant',
];
export async function addRec() {
    const rc = config.collection;
    const rt = config.rectype;

    console.log(`Action >> [fn]: 'addRec'... [collection]: `, rc)
    console.log(`Action >> [fn]: 'addRec'... [rectype]: `, rt)

    let valid = false

    for (const addtype of addableTypes){
        if (rt === addtype){
            valid = true
            break
        }
    }

    if (valid) {
    /* Transactions */
        if (rt === "Expense"){
            const rec = rc as Expense;
            await addExpense(rec);
            await getTable("Transactions");
            await getTable("Accounts");
        } else if (rt === "Deposit"){
            const rec = rc as Deposit;
            await addDeposit(rec);
            await getTable("Transactions");
            await getTable("Accounts");
        } else if (rt === "Transfer"){
            const rec = rc as Transfer;
            await addTransfer(rec);
            await getTable("Transactions");
            await getTable("Accounts");
        } else
    /* Accounts */ 
        if (rt === "Mobile Account"){
            const rec = rc as MobileAccount;
            await addMobileAccount(rec);
            await getTable("Accounts");
        } else if (rt === "Credit Card"){
            const rec = rc as CreditCard;
            await addCreditCard(rec);
            await getTable("Accounts");
        } else if (rt === "Checking Account"){
            const rec = rc as CheckingAccount;
            await addCheckingAccount(rec);
            await getTable("Accounts");
        } else if (rt === "Savings Account"){
            const rec = rc as SavingsAccount;
            await addSavingsAccount(rec);
            await getTable("Accounts");
        } else if (rt === "Investment Account"){
            const rec = rc as InvestmentAccount;
            await addInvestmentAccount(rec);
            await getTable("Accounts");
        } else
    /* Budgets */
        if (rt === "Budget"){
            const rec = rc as Budget;
            const mm = rec.month;
            const yyyy = rec.year;
            const budg: NewBudgetRec = {month: mm, year: yyyy};
            console.log("budg: ", budg)
            const bid = await addBudget(budg);
            const bcats = rec.categories as BudgetCategory[];
            for(const bcat of bcats){
                const nbcat: NewBudgetCategory = {budget_id: bid, category_id: bcat.category_id, calc_type: bcat.calc_type, spending_limit: bcat.spending_limit}
                await addCategoryToBudget(nbcat);
            }
            await getTable("Budgets");
        } else 
    /* Others */
        if (rt === "Category"){
            const rec = rc as Category;
            await addCategory(rec);
            await getTable("Categories");
        } else if (rt === "Depositor"){
            const rec = rc as Depositor;
            await addDepositor(rec);
            await getTable("Depositors");
        } else if (rt === "User"){
            const rec = rc as User;
            await addUser(rec);
            await getTable("Users");
        } else if (rt === "Merchant"){
            const rec = rc as Merchant;
            await addMerchant(rec);
            await getTable("Merchants");
        } else {
            console.log(missingLogicErrorMsg)
        }
    } else {
        console.log(addRecErrorMsg)
    }
    closeForm();
}