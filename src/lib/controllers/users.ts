import type { User, Users } from '$lib/models';
import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

/* Stores ----------------------------------------------------------------------------------------------- */
export const UsersStore: Writable<Users> = writable()
export const CurrentUser: Writable<User> = writable();

/** Dev User Only                                **/
/** -------------------------------------------- **/
/**/    const dev: User = {                     /**/
/**/        id: 2,                              /**/
/**/        name: "Joe",                        /**/
/**/        last_name: "Dirt",                  /**/ 
/**/        username: "jdirt",                  /**/
/**/        pin: 123456,                        /**/
/**/        password: `password`,               /**/
/**/        icon: "/users/default.png",         /**/
/**/        icon_color: "#000000",              /**/
/**/        require_auth: 0,                    /**/
/**/    }                                       /**/
/**/    CurrentUser.set(dev)                    /**/
/** -------------------------------------------- **/

/* Read ----------------------------------------------------------------------------------------------- */
export async function getUsers(){
    const resp: Users = await invoke('cmd_get_users')
    UsersStore.set(resp)
    console.log("DB Fetch: Users");
}

/* Create ----------------------------------------------------------------------------------------------- */
export async function addUser(value: User){
    console.log("Attempting to Add User: ", value);
    const resp = await invoke('cmd_add_user', { user: value });
    console.log("Add User status: ", resp);
}

/* Update ----------------------------------------------------------------------------------------------- */
export async function editUser(values: User, userID: number) {
    console.log(`Updating User: ${userID}, value = `, values)
    const resp = await invoke('cmd_edit_users', {u: values, id: userID})
    console.log(`User update status: `, resp)
}