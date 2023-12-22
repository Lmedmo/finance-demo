import type { Merchant, Merchants } from '$lib/models';
import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

/* Stores ----------------------------------------------------------------------------------------------- */
export const MerchantsStore: Writable<Merchants> = writable()

/* Read ----------------------------------------------------------------------------------------------- */
export async function getMerchants(){
    const resp: Merchants = await invoke('cmd_get_merchants');
    MerchantsStore.set(resp)
    console.log("DB Fetch: Merchants");
}

/* Create ----------------------------------------------------------------------------------------------- */
export async function addMerchant(value: Merchant){
    console.log("Attempting to Add Merchant: ", value);
    const resp = await invoke('cmd_add_merchant', { merchant: value });
    console.log("Add Merchant Status: ", resp);
}

/* Update ----------------------------------------------------------------------------------------------- */
export async function editMerchant(values: Merchant, merch_id: number) {
    console.log(`Updating Merchant: ${merch_id}, value = `, values)
    const resp = await invoke('cmd_edit_merchant', {m: values, id: merch_id})
    console.log(`Merchant update status: `, resp)
}