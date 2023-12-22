import type { Transaction, Account } from "$lib";
import { FormConfig, closeForm, getTable, showForm, type RecordType, type Explorer, type FormConfigObj } from "$lib/config";
import { missingLogicErrorMsg, deleteRecErrorMsg } from "$lib/config/errors";
import { deleteTransactions, deleteAccount } from "$lib/controllers";

let config: FormConfigObj;
FormConfig.subscribe((value)=>{ config = value });

export function initDeleteForm(recordType: RecordType, explCtx: Explorer){
    console.log(`Action >> [fn]: 'initDeleteForm'... [args]: 'recordType' = `, recordType)
    console.log(`Action >> [fn]: 'initDeleteForm'... [args]: 'explCtx' = `, explCtx)

    let rtype;
    for(const rectype of deleteableTypes){
        if (rectype === recordType){
            rtype = rectype;
            break
        }
    }
    if (typeof rtype !== 'undefined'){
        config.formtype = "Delete"
        config.rectype = rtype
        config.explorer = explCtx
        FormConfig.set(config)
        showForm()
    } else {
        console.log("Type is not Deletable")
    }
}
export const deleteableTypes: RecordType[] = [
// Transactions
    'Transactions',
    'Transaction', 
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
];
export async function deleteRecs() {
    const rs = config.subjects;
    const rt = config.rectype;

    console.log(`Action >> [fn]: 'deleteRecs'... [detail]: rt = `, rt);
    console.log(`Action >> [fn]: 'deleteRecs'... [detail]: rs = `, rs);

    let valid = false
    for (const deletetype of deleteableTypes){
        if (rt === deletetype){
            valid = true
            break
        }
    }
    if (valid) {
        if (rt === "Transaction" || rt === "Expense" || rt === "Deposit" || rt === "Transfer" || rt === "Transactions"){
            const records = rs as Transaction[]
            const ids: number[] = [];
            for (const record of records){
                const val = record.id as number;
                ids.push(val);
            }
            await deleteTransactions(ids);
            await getTable("Transactions");
            await getTable("Accounts");   
        } else if (rt === "Account" || rt === "Checking Account" || rt === "Savings Account" || rt === "Mobile Account" || rt === "Investment Account" || rt === "Credit Card"){
            const account = rs[0] as Account
            const acctID = account.id as number;
            await deleteAccount(acctID);
            await getTable("Accounts");
            await getTable("Transactions");
        } else {
            console.log(missingLogicErrorMsg)
        }
    } else {
        console.log(deleteRecErrorMsg)
    }
    closeForm();
}