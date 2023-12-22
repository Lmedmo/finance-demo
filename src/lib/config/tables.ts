import { getAccounts, getBudgets, getCategories, getDepositors, getMerchants, getTransactions, getUsers } from "$lib/controllers";
//import { ReloadPending } from "./app";
import { FormConfig, type FormConfigObj, type SelectedRecord } from "./form";

/* Types ------------------------------------------------------------------------------------------------ */
export interface TableFilter {
    table: TableStore;
    fieldID: FilterField;
    values: number[];
}
export interface TableColumn {
    fieldID: string;
    headingname: string;
}
export type TableStore = 
    "Transactions" | "Accounts" | "Categories" | "Merchants" | 
    "Depositors" | "Budgets" | "Banks" | "Users" | "";

export type FilterField = "type_id" | "memo_id" | "none";

/* Functions -------------------------------------------------------------------------------------------- */
//let current: boolean;
let config: FormConfigObj;
FormConfig.subscribe((value)=>{config = value})
//ReloadPending.subscribe((value)=>{current=value});

export async function getTables(){
    console.log(`table controller >> [fn]: 'getTables'`)
    console.log("Fetching Tables...")
    await getTransactions();
    await getAccounts();
    await getCategories();
    await getMerchants();
    await getDepositors();
    await getBudgets();
    await getUsers();
}
export async function getTable(tablename: string){
    console.log(`table controller >> [fn]: 'getTable'; [arg]: 'tablename' = `, tablename)
    console.log("Fetching table: ", tablename)
    if (tablename === "Transactions") {
        await getTransactions();
        console.log("TransactionsStore Updated");
    } else if (tablename === "Accounts"){
        await getAccounts();
        console.log("AccountsStore Updated");
    } else if (tablename === "Categories"){
        await getCategories();
        console.log("CategoriesStore Updated");
    } else if (tablename === "Merchants"){
        await getMerchants();
        console.log("MerchantsStore Updated");
    } else if (tablename === "Depositors"){
        await getDepositors();
        console.log("DepositorStore Updated");
    } else if (tablename === "Budgets"){
        await getBudgets();
        console.log("BudgetsStore Updated");
    } else if (tablename === "Users"){
        await getUsers();
        console.log("UsersStore Updated");
    } else {
        console.log("Error retreiving data: Tablename does not exist, or operation invalid.");
    }
    //ReloadPending.set(!current);
}
export function toggleSelected(rec: SelectedRecord){
    console.log(`table controller >> [fn]: 'toggleSelected'... [args]: 'rec'...`, rec)
    const subs = config.subjects;
    const selected: SelectedRecord[] = [];
    let included: boolean = false;
    
    if (subs.length == 0){
        selected.push(rec);
    } else {
    /*  Add all previously selected records to the selected array unless they're the record 
        being toggled, in which case set 'included' = true */
        for(const sub of subs){
            if(sub.id == rec.id){
                included = true;
            } else {
                selected.push(sub)
            }
        }
    /*  Adds the toggled record to the selected array if it was not included (leaving it out 
        if it was included, effectively toggling it) */
        if (!included){
            selected.push(rec)
        }
    }
    config.subjects = selected
    FormConfig.set(config);
    console.log(`form controller >> [msg]: 'FormConfig.subjects updated'... [details]: `, config)
}