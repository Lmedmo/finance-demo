import type { Account, Transaction } from "$lib";
import type { ExplOption } from "$lib/config/explorer";

export type Formulas = "Balance" | "Average" | "Net-Worth";

export function calcBalance(data: Account[], filter: ExplOption){
    console.log("fn call: calcBalance")
    let bal = 0.00;
    for (const item of data){
        const id = item.id
        if (id == filter.id){
            bal = item.balance as number;
        }
    }
    return bal
}
export function calcAverage(data: Transaction[], filter: ExplOption){
    let balance: number = 0;
    let averageValue: number = 0;
    let count: number = 0
    for(const record of data){
        if (filter.id == 0){
            balance += record.amount;
        } else if (record.account_id === filter.id){
            balance += record.amount;
        } else {
            balance += 0.00
        }
        count += 1
    }
    averageValue = balance/count
    return averageValue;
}
export function calcNetWorth(data: Account[]){
    console.log("fn call: calcNetWorth")
    let net = 0.00;
    for (const item of data){
        net += item.balance as number;
    }
    return net;
}