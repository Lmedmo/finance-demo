import { DepositorStore } from "$lib/controllers";

export const emptyDepositor: Depositor = {id: 0, name: ''};


let deposstr: Depositors;
DepositorStore.subscribe((value)=>{ deposstr = value });

export function getDepositor(id: number){
    let depositor = emptyDepositor
    for (const depos of deposstr.records){
        if (depos.id == id){
            depositor = depos
            break
        }
    }
    const resp = [depositor]
    return resp
}
export interface Depositor {
    [index: string]: string | number | undefined;
    id?: number; 
    name: string;
}
export interface Depositors {
    records: Depositor[];
}