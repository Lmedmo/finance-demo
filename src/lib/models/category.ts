import { CategoriesStore } from "$lib/controllers";

export const emptyCategory: Category = {
    id: 0,
    name: "",
    icon: "",
    icon_color: "",
    type_id: 1,
}

let catstr: Categories;
CategoriesStore.subscribe((value)=>{ catstr = value });

export function getCategory(id: number){
    let category = emptyCategory
    for (const cat of catstr.records){
        if (cat.id == id){
            category = cat
            break
        }
    }
    const resp = [category]
    return resp
}
export interface Category {
    [index: string]: string | number | undefined;
    id: number; 
    name: string;
    icon: string;
    icon_color: string;
    type_id?: number;
}
export interface NewCategory {
    [index: string]: string | number | undefined;
    name: string;
    icon: string;
    icon_color: string;
    type_id?: number;
}
export interface Categories {
    records: Category[];
}