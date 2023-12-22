import { FormConfig, closeForm, defaultCollection, getTable, showForm, type Explorer, type FormConfigObj } from "$lib/config";
import { invoke } from "@tauri-apps/api";
import { downloadDir } from "@tauri-apps/api/path";
import { open } from '@tauri-apps/api/dialog';

export async function importStatement(accountID: number, explCtx: Explorer) {
    const msg = "'accountID' = " + accountID + "... 'explCtx' = " + explCtx;
    console.log(`Action >> [fn]: 'importStatement'... [args]: `, msg)

    const configuration: FormConfigObj = {
        formtype: "Import", 
        rectype: "Statement", 
        subjects: [], 
        collection: defaultCollection, 
        explorer: explCtx
    }
    FormConfig.set(configuration)
    showForm()
    const selected = await open({
        title: "Import Statement",
        defaultPath: await downloadDir(),
        filters: [{
            name: 'Statement',
            extensions: ['csv'],
        }]
    })
    console.log("Selected file: ", selected)

    if (selected !== null){
        await invoke('import_statement', { acct: accountID, csvpath: selected });
        await getTable("Transactions");
        await getTable("Accounts");
    }
    closeForm()
}