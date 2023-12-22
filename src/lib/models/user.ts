export const emptyUser: User = {
    id: 0,
    name: '',
    last_name: '',
    username: '',
    pin: 123456,
    password: '',
    icon: '',
    icon_color: '',
    require_auth: ''
}
export interface User {
    [index: string]: string | number | undefined;
    id: number; 
    name: string; 
    last_name?: string|number; 
    username?: string|number;
    pin?: string|number; 
    password?: string|number; 
    icon: string;
    icon_color: string;
    require_auth?: string | number;
}
export interface Users {
    records: User[];
}