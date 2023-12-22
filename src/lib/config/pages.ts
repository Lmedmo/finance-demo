import { writable, type Writable } from 'svelte/store';
import AppConfig from '../../routes/app.config.json';

/* Stores ----------------------------------------------------------------------------------------------- */
export const PageState: Writable<Page> = writable();

/* Types ----------------------------------------------------------------------------------------------- */
export interface Page {
    id: number;
    title: string;
}

/* Functions ----------------------------------------------------------------------------------------------- */
export function fetchPages(){
    const pages = [];
    for (const page of AppConfig.Pages){
        if (page.pageID > 0){
            const pageOpt = {id: page.pageID, title: page.Name}
            pages.push(pageOpt)
        }
    }
    console.log(`Pages Fetch... PageState updated to: `, pages[0])
    PageState.set(pages[0])
    return pages;
}