import type { Categories, Category } from '$lib/models';
import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

/* Stores ----------------------------------------------------------------------------------------------- */
export const CategoriesStore: Writable<Categories> = writable()

/* Read ----------------------------------------------------------------------------------------------- */
export async function getCategories(){
    const resp: Categories = await invoke('cmd_get_categories');
    CategoriesStore.set(resp)
    console.log("DB Fetch: Categories");
}

/* Create ----------------------------------------------------------------------------------------------- */
export async function addCategory(value: Category){
    console.log("Attempting to Add Category: ", value);
    const resp = await invoke('cmd_add_category', { category: value });
    console.log("Add Category Status: ", resp);
}

/* Delete ----------------------------------------------------------------------------------------------- */
export async function deleteCategory(id: number){
    console.log("Deleting Category: ", id)
    const resp = await invoke("cmd_delete_custom_category", { id: id } );
    console.log("Deletion Status: ", resp);
}

/* Update ----------------------------------------------------------------------------------------------- */
export async function editCategory(values: Category, catID: number){
    console.log("Updating Category: ", values);
    const resp = await invoke('cmd_edit_goal', { c: values, id: catID });
    console.log("Category update status: ", resp);
}
