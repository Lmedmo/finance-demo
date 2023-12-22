export const emptyTransaction: Transaction = {
    type_id: 0,
    type_name: '',
    memo_id: 0,
    memo_name: '',
    account_id: 0,
    account_name: '',
    to_from_account: 0,
    to_from_acct_name: '', 
    date: '',
    merchant_id: 0,
    merchant_name: '',
    depositor_id: 0,
    depositor_name: '',
    description: '', 
    category_id: 0,
    category_name: '',
    goal_id: 0,
    goal_name: '',
    amount: 0.00
}
export const emptyDeposit: Deposit = {
    type_id: 0,
    type_name: '',
    memo_id: 0,
    memo_name: '',
    account_id: 0,
    account_name: '',
    date: '',
    depositor_id: 0,
    depositor_name: '',
    description: '', 
    amount: 0.00
}
export const emptyExpense: Expense = {
    //id?: number;
    type_id: 0,
    type_name: '',
    memo_id: 0,
    memo_name: '',
    account_id: 0,
    account_name: '',
    date: '',
    merchant_id: 0,
    merchant_name: '',
    description: '', 
    category_id: 0,
    category_name: '',
    amount: 0.00
}
export const emptyTransfer: Transfer = {
    type_id: 0,
    type_name: '',
    memo_id: 0,
    memo_name: '',
    account_id: 0,
    account_name: '',
    to_from_account: 0,
    to_from_acct_name: '', 
    date: '',
    description: '', 
    amount: 0.00
}
export interface Transaction {
    [index: string]: string | number | undefined;
    id?: number;
    type_id: number;
    type_name: string;
    memo_id: number;
    memo_name: string;
    account_id: number;
    account_name: string;
    to_from_account?: number;
    to_from_acct_name?: string; 
    date: string;
    merchant_id?: number;
    merchant_name?: string;
    depositor_id?: number;
    depositor_name?: string;
    description?: string; 
    category_id?: number;
    category_name?: string;
    goal_id?: number;
    goal_name?: string;
    amount: number;
}
export interface Deposit {
    [index: string]: string | number | undefined;
    type_id: number;
    type_name: string;
    memo_id: number;
    memo_name: string;
    account_id: number;
    account_name: string;
    date: string;
    depositor_id?: number;
    depositor_name?: string;
    description?: string; 
    amount: number;
}
export interface Expense {
    [index: string]: string | number | undefined;
    type_id: number;
    type_name: string;
    memo_id: number;
    memo_name: string;
    account_id: number;
    account_name: string;
    date: string;
    merchant_id?: number;
    merchant_name?: string;
    description?: string; 
    category_id?: number;
    category_name?: string;
    amount: number;
}
export interface Transfer {
    [index: string]: string | number | undefined;
    type_id: number;
    type_name: string;
    memo_id: number;
    memo_name: string;
    account_id: number;
    account_name: string;
    to_from_account?: number;
    to_from_acct_name?: string; 
    date: string;
    description?: string; 
    amount: number;
}
export interface Transactions {
    records: Transaction[];
    types: TransactionType[];
    memos: MemoType[];
}
export interface TransactionType {
    id: number;
    name: string;
}
export interface MemoType {
    id: number;
    name: string;
}