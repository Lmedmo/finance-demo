import { AccountsStore } from "$lib/controllers"

export const emptyAcctStr: Accounts = {
    records: [],
    types: [],
    cfreqs: [],
    punits: [],
    banks: [],
}
export const emptyAccount: Account = { 
    name: '',
    bank_id: 0,
    user_id: 0,
    type_id: 0,
    initial_balance: 0,
    credit_limit: 0,
    due_date: '',
    interest_rate: 0,
    compound_frequency: 0,
    period_unit: '',
    account_username: '',
    account_number: '',
    routing_number: '',
    balance: 0.00
}
export const emptyCreditCard: CreditCard = {
    name: '',
    bank_id: 0,
    user_id: 0,
    type_id: 1,
    initial_balance: 0,
    credit_limit: 0,
    due_date: '',
    interest_rate: 0,
    compound_frequency: undefined,
    period_unit: undefined,
    account_username: undefined,
    account_number: undefined,
    routing_number: undefined
}
export const emptyMobileAcct: MobileAccount = {
    name: '',
    bank_id: 0,
    user_id: 0,
    type_id: 2,
    initial_balance: 0,
    credit_limit: undefined,
    due_date: undefined,
    interest_rate: undefined,
    compound_frequency: undefined,
    period_unit: undefined,
    account_username: '',
    account_number: undefined,
    routing_number: undefined
}
export const emptyChecking: CheckingAccount = { 
    name: '',
    bank_id: 0,
    user_id: 0,
    type_id: 3,
    initial_balance: 0,
    credit_limit: undefined,
    due_date: undefined,
    interest_rate: undefined,
    compound_frequency: undefined,
    period_unit: undefined,
    account_username: undefined,
    account_number: '',
    routing_number: ''
}
export const emptySavings: SavingsAccount = {
    name: '',
    bank_id: 0,
    user_id: 0,
    type_id: 4,
    initial_balance: 0,
    credit_limit: undefined,
    due_date: undefined,
    interest_rate: 0,
    compound_frequency: 0,
    period_unit: 'None',
    account_username: undefined,
    account_number: '',
    routing_number: ''
}
export const emptyInvestment: InvestmentAccount = {
    name: '',
    bank_id: 0,
    user_id: 0,
    type_id: 5,
    initial_balance: 0,
    credit_limit: undefined,
    due_date: undefined,
    interest_rate: undefined,
    compound_frequency: undefined,
    period_unit: undefined,
    account_username: undefined,
    account_number: undefined,
    routing_number: undefined
}
export const emptyBank: Bank = {
    id: 6,
    name: '',
    icon: '',
    icon_color: ''
}

let acctstr: Accounts;
AccountsStore.subscribe((value)=>{ acctstr = value });

export function getAccount(id: number){
    let account = emptyAccount
    for (const acct of acctstr.records){
        if (acct.id == id){
            account = acct
            break
        }
    }
    const resp = [account]
    return resp
}
export function getAccountType(id: number){
    let acctype: AccountType = {id: 0, name: ''}
    for (const atype of acctstr.types){
        if (atype.id == id){
            acctype = atype
            break
        }
    }
/* ['Credit Card', 'Mobile Account', 'Checking Account', 'Savings Account', 'Investment Account', 'Other'] */
    const resp = acctype.name
    return resp
}
export function getBank(id: number){
    let bankRec: Bank = {id: 1, name: 'Other', icon: "/banks/bank.png", icon_color: "#00DB99"}
    for(const bnk of acctstr.banks){
        if (bnk.id == id){
            bankRec = bnk
            break
        }
    }
    return bankRec
}
export function getCompFreq(optVal: number){
    let cfRec: CompoundFreqOpt = {id: 0, name: 'None'}
    for(const opt of acctstr.cfreqs){
        if (opt.id == optVal){
            cfRec = opt
            break
        }
    }
    return cfRec
}
export function getPeriodUnit(name: string){
    let puRec: PeriodUnitType = {id: 0, name: 'None'}
    for(const opt of acctstr.punits){
        if (opt.name === name){
            puRec = opt
            break
        }
    }
    return puRec
}

export interface CreditCard {
    [index: string]: string | number | undefined;
    id?: number; 
    name: string;
    bank_id: number;
    user_id: number;
    type_id: number;
    initial_balance: number;
    credit_limit?: number;
    due_date?: string;
    interest_rate?: number;
}
export interface MobileAccount {
    [index: string]: string | number | undefined;
    id?: number; 
    name: string;
    bank_id: number;
    user_id: number;
    type_id: number;
    initial_balance: number;
    account_username?: string;
}
export interface CheckingAccount {
    [index: string]: string | number | undefined;
    id?: number; 
    name: string;
    bank_id: number;
    user_id: number;
    type_id: number;
    initial_balance: number;
    account_number?: string;
    routing_number?: string;
}
export interface SavingsAccount {
    [index: string]: string | number | undefined;
    id?: number; 
    name: string;
    bank_id: number;
    user_id: number;
    type_id: number;
    initial_balance: number;
    interest_rate?: number;
    compound_frequency?: number;
    period_unit?: string;
    account_number?: string;
    routing_number?: string;
}
export interface InvestmentAccount {
    [index: string]: string | number | undefined;
    id?: number; 
    name: string;
    bank_id: number;
    user_id: number;
    type_id: number;
    initial_balance: number;
}
export interface Account {
    [index: string]: string | number | undefined;
    id?: number; 
    name: string;
    bank_id: number;
    user_id: number;
    type_id: number;
    initial_balance: number;
    credit_limit?: number;
    due_date?: string;
    interest_rate?: number;
    compound_frequency?: number;
    period_unit?: string;
    account_username?: string;
    account_number?: string;
    routing_number?: string;
    balance?: number;
}
export interface NewAccount {
    [index: string]: string | number | undefined;
    id?: number; 
    name: string;
    bank_id: number;
    user_id: number;
    type_id: number;
    initial_balance: number;
    credit_limit?: number;
    due_date?: string;
    interest_rate?: number;
    compound_frequency?: number;
    period_unit?: string;
    account_username?: string;
    account_number?: string;
    routing_number?: string;
}
export interface Accounts {
    records: Account[];
    types: AccountType[];
    cfreqs: CompoundFreqOpt[];
    punits: PeriodUnitType[];
    banks: Bank[];
}
export interface AccountType {
    [index: string]: string | number;
    id: number;
    name: string;
}
export interface PeriodUnitType {
    [index: string]: string | number | undefined;
    id: number;
    name: string;
}
export interface CompoundFreqOpt {
    [index: string]: string | number | undefined;
    id: number;
    name: string;
}
export interface Bank {
    [index: string]: string | number | undefined;
    id: number; 
    name: string;
    icon: string;
    icon_color: string;
}
