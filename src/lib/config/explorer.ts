import { writable, type Writable } from 'svelte/store';
import AppConfig from '../../routes/app.config.json';
import type { Page } from './pages';
import type { View } from './views';
import type { Account, Accounts, Budget, Budgets, Categories, Category, Depositor, Depositors } from '$lib/models';
import type { TableStore } from './tables';

/* Stores ----------------------------------------------------------------------------------------------- */
export const ExplorerState: Writable<Explorer> = writable();

/* Types ----------------------------------------------------------------------------------------------- */
export interface Explorer {
    shown: boolean;
    references: TableStore;
    selected: ExplOption;
}
export interface ExplOption {
    id: number;
    name: string;
    type: number;
    icon: string;
    icon_color: string;
}

/* Functions ----------------------------------------------------------------------------------------------- */
export async function fetchExplorer(currentpage: Page, currentview: View){
    let explState: boolean = false;
    let ref = "";
    let defOpt: ExplOption = {id: 0, name: "", type: 0, icon: '', icon_color: ''};

    for (const page of AppConfig.Pages){
        if (page.pageID === currentpage.id){
            for(const view of page.Views){
                if (view.viewID === currentview.id){
                    if (view.Explorer.included){
                        explState = true
                    }
                    explState = view.Explorer.included
                    ref = view.Explorer.references
                    defOpt = view.Explorer.defaultOption
                    if (defOpt.icon === "" || defOpt.icon === "Default"){
                        defOpt.icon = '/icons/icon.png'
                    }
                    if (defOpt.icon_color === "" || defOpt.icon_color === "Default"){
                        defOpt.icon_color = AppConfig.AppColor
                    }
                } 
            }
        }
    }
    const expl: Explorer = {shown: explState, references: ref as TableStore, selected: defOpt}
    ExplorerState.set(expl)
    console.log(`Explorer Fetch... ExplorerState updated to: `, expl)
    return expl
}
export function getOpts(rectype: string, def: ExplOption, store: Accounts | Categories | Depositors | Budgets ){
    console.log(`explorer controller >> [fn]: 'getOpts'`)
    console.log(`explorer controller >> [arg]: 'rectype' = `, rectype)
    console.log(`explorer controller >> [arg]: 'def' = `, def)
    console.log(`explorer controller >> [arg]: 'store' = `, store)
    
    const opts = []
    const recs = store.records
    let icon = '';
    let icon_color = '';
    let opt: ExplOption;
    opts.push(def)

    if (recs.length > 0){
        let records: Account[] | Category[] | Depositor[] | Budget[] = [];
        if (rectype === "Account"){
            records = recs as Account[];

            const acctstore = store as Accounts;
            const banks = acctstore.banks;
            
            for(const record of records){
                const bank_id = record.bank_id;
                const acct_id = record.id as number;
                const acct_name = record.name;
                const acct_type = record.type_id as number;
                for(const bank of banks){
                    if (bank.id == bank_id){
                        icon = bank.icon;
                        icon_color = bank.icon_color;
                        break
                    }
                }
                if (icon === "" || icon === "Default"){
                    icon = '/banks/bank.png'
                }
                if (icon_color === "" || icon_color === "Default"){
                    icon_color = AppConfig.AppColor
                }
                opt = {id: acct_id, name: acct_name, type: acct_type, icon: icon, icon_color: icon_color}
                opts.push(opt)
            }
        } else if (rectype === "Category"){
            records = recs as Category[];
            for(const record of records){
                const cat_id = record.id as number
                const cat_name = record.name
                if (record.icon === "" || record.icon === "Default"){
                    icon = "/icons/icon.png"
                } else {
                    icon = record.icon as string
                }
                if (record.icon_color === "" || record.icon_color === "Default"){
                    icon_color = AppConfig.AppColor
                } else {
                    icon_color = record.icon_color as string
                }
                opt = {id: cat_id, name: cat_name, type: 0, icon: icon, icon_color: icon_color}
                opts.push(opt)
            }
        } else if (rectype === "Depositor"){
            records = recs as Depositor[];
            for(const record of records){
                const depos = record as Depositor;
                const id = depos.id as number;
                icon = "/icons/icon.png"
                icon_color = AppConfig.AppColor
                opt = {id: id, name: depos.name, type: 0, icon: icon, icon_color: icon_color}
                opts.push(opt)
            }
        } else if (rectype === "Budget"){
            records = recs as Budget[];
            for(const record of records){
                const budg = record as Budget;
                const budg_id = budg.id as number;
                const budg_name = `${budg.month} ${budg.year}`
                icon = "/icons/icon.png"
                icon_color = AppConfig.AppColor
                opt = {id: budg_id, name: budg_name, type: 0, icon: icon, icon_color: icon_color}
                opts.push(opt)
            }
        }
    }
    console.log("Explorer Options: ", opts)
    return opts
}
