export const emptyMerchant: Merchant = {id: 0, name: "", icon: "", icon_color: ""}
export interface Merchant {
    [index: string]: string | number | undefined;
    id?: number; 
    name: string;
    icon: string;
    icon_color: string;
}
export interface Merchants {
    records: Merchant[];
}