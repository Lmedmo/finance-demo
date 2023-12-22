import type { Depositor, Depositors } from '$lib/models';
import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

/* Stores ----------------------------------------------------------------------------------------------- */
export const DepositorStore: Writable<Depositors> = writable()

/* Read ----------------------------------------------------------------------------------------------- */
export async function getDepositors(){
    const resp: Depositors = await invoke('cmd_get_depositors');
    DepositorStore.set(resp)
    console.log("DB Fetch: Depositors... Response: ", resp);
}

/* Create ----------------------------------------------------------------------------------------------- */
export async function addDepositor(value: Depositor){
    console.log("Attempting to Add Depositor: ", value);
    const resp = await invoke('cmd_add_depositor', { depositor: value });
    console.log("Add Depositor Status: ", resp);
}

/* Update ----------------------------------------------------------------------------------------------- */
export async function editDepositor(values: Depositor, depositorID: number) {
    console.log(`Updating Depositor: ${depositorID}, value = `, values)
    const resp = await invoke('cmd_edit_depositor', {d: values, id: depositorID})
    console.log(`Depositor update status: `, resp)
}