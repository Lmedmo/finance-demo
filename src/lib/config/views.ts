import { writable, type Writable } from 'svelte/store';
import AppConfig from '../../routes/app.config.json';
import type { Page } from './pages';

/* Stores ----------------------------------------------------------------------------------------------- */
export const ViewState: Writable<View> = writable();

/* Types ----------------------------------------------------------------------------------------------- */
export interface View {
    id: number;
    title: string;
}

/* Functions ----------------------------------------------------------------------------------------------- */
export function fetchViews(currentpage: Page){
    const views = [];
    for (const page of AppConfig.Pages){
        if (page.pageID === currentpage.id && page.pageID >= 0){
            for(const view of page.Views){
                const viewOpt = {id: view.viewID, title: view.Name}
                views.push(viewOpt)
            }
            break
        }
    }
    console.log(`Views Fetch... ViewState updated to: `, views[0])
    ViewState.set(views[0])
    return views;
}