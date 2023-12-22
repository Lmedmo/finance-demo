import { getCurrent, LogicalSize, currentMonitor, appWindow, type Theme } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api'; //setMinSize
import { platform } from '@tauri-apps/api/os';
import { writable, type Writable } from 'svelte/store';
import AppConfig from '../../routes/app.config.json';
import { Colors } from '$lib';

/* Stores ----------------------------------------------------------------------------------------------- */
export const AppState: Writable<AppStateObj> = writable();
export const ReloadPending = writable(false);

/* Types ------------------------------------------------------------------------------------------------ */
export interface AppStateObj {
    themeval: string;
    theme: string;
    explorer: boolean;
    os: string;
    font: string;
    color: string;
    text: string;
    fill: string;
    window_clr: string;
    tb_txtclr: string;
    set_fill: string;
    rib_expl_btn: string;
    expl_fill: string;
    view_fill: string;
    tool_btn: string;
    table_row: string;
    viewlet: string;
}
export interface AppSettings {
    id: number;
    theme: string;
    font: string;
}
export interface SqliteQueryResult {
    changes: number;
    last_insert_rowid: number;
}

/* Functions ----------------------------------------------------------------------------------------------- */
export async function getAppState() {
    const app: AppSettings = await invoke('cmd_get_appstate');
    const os = await platform();

    const font = app.font;
    const themeval = app.theme;
    const expl = true; //app.explorer;
    const appclr = AppConfig.AppColor;

    let theme;
    if (themeval === 'Default'){
        const currentSys = await appWindow.theme();
        theme = currentSys as Theme;
    } else {
        theme = themeval
    }
    let tbtxtcolor = "";
    let winclr = '';
    let setclr = '';
    let ribexbtn = '';
    let explfill = '';
    let vwfill = '';
    let tlbtn = '';
    let tblrow = '';
    let vwlet = '';
    let fillclr = '';

    if (theme === 'light'){
        winclr = `  background: linear-gradient(0deg, #ffffff99 0%, #ffffff99 100%), linear-gradient(0deg, ${appclr}b3 0%, ${appclr}b3 100%), #FFF;
                    backdrop-filter: blur(10px);`;
        tbtxtcolor = Colors.charcoal;
        setclr = "rgba(190, 189, 191, 0.30);";
        ribexbtn = Colors.white;
        explfill = 'border: 1px solid #ffffff80; background: #ffffff4d;';
        vwfill = Colors.white;
        tlbtn = `background: #ffffff80;`;
        tblrow = `background: rgba(190, 189, 191, 0.30); text-shadow: 0px 0px 4px rgba(255, 255, 255, 0.7); border-top: 1px solid rgb(0, 0, 0, 15%); border-bottom: 1px solid rgb(0, 0, 0, 15%);`;
        vwlet = `rgba(190, 189, 191, 0.30);`;
        fillclr = Colors.white;
    } else if (theme === 'dark'){
        winclr = `  background: linear-gradient(0deg, #00000099 0%, #00000099 100%), linear-gradient(0deg, ${appclr}99 0%, ${appclr}99 100%), #FFF;
                    backdrop-filter: blur(2px);`;
        tbtxtcolor = Colors.white;
        setclr = "rgba(0, 0, 0, 0.30);";
        ribexbtn = Colors.charcoal;
        explfill = 'border: 1px solid #4d4d4d26; background: linear-gradient(0deg, #ffffff4d 0%, #ffffff4d 100%), #4d4d4d4d;';
        vwfill = Colors.charcoal;
        tlbtn = `background: #4d4d4d80;`;
        tblrow = `background: rgba(255, 255, 255, 0.40); text-shadow: 0px 0px 4px rgba(0, 0, 0, 0.2); border-top: 1px solid rgb(255, 255, 255, 15%); border-bottom: 1px solid rgb(255, 255, 255, 15%);`;
        vwlet = `rgba(0, 0, 0, 0.30);`;
        fillclr = Colors.charcoal;
    }

    const appstate: AppStateObj = {
        themeval: themeval,
        theme: theme,
        color: appclr,
        os: os,
        font: font,
        text: tbtxtcolor,
        explorer: expl,
        fill: fillclr,
        window_clr: winclr,
        tb_txtclr: `color: ${tbtxtcolor}; `,
        set_fill: setclr,
        rib_expl_btn: ribexbtn,
        expl_fill: explfill,
        view_fill: vwfill,
        tool_btn: tlbtn,
        table_row: tblrow,
        viewlet: vwlet
    }
    console.log("appstate: ", appstate)
    AppState.set(appstate);
}
export async function getWindowSize() {
    const monitor = await currentMonitor();
    const mSize = monitor?.size;
    const mScale = monitor?.scaleFactor;

    console.log("monitor: ", monitor);
    console.log("mSize: ", mSize);

    let wSizeX = 1520;
    let wSizeY = 980;
    
    if (typeof mSize !== 'undefined' && typeof mScale !== 'undefined'){
        const mSizeX = mSize.width/mScale;
        const mSizeY = mSize.height/mScale;
        const ratio = mSizeX / mSizeY;
        if (ratio > 2.0){
            wSizeX = Math.floor(.5 * mSizeX);
            wSizeY = Math.floor(.8 * mSizeY);
        } else {
            wSizeX = Math.floor(.8 * mSizeX);
            wSizeY = Math.floor(.85 * mSizeY);
        }
    }
    await getCurrent().setSize(new LogicalSize(wSizeX, wSizeY));
    await getCurrent().center()
}
export async function changeTheme(name: string, current: AppStateObj){
    console.log("changeTheme(): name=", name)
    console.log("changeTheme(): current=", current)
    let value: AppSettings;

    if (name === 'light'){
        value = {id: 1, theme: 'light', font: current.font}
    } else if (name === 'dark'){
        value = {id: 1, theme: 'dark', font: current.font}
    } else {
        value = {id: 1, theme: 'Default', font: current.font}
    }

    await invoke('cmd_set_appstate', { conf: value })
    await getAppState()
}
export async function changeFont(name: string, current: AppStateObj){
    console.log("changeFont(): name=", name)
    console.log("changeFont(): current=", current)
    let value: AppSettings;

    if (name === 'Quicksand'){
        value = {id: 1, theme: current.themeval, font: 'Quicksand'}
    } else if (name === 'Avenir-Next'){
        value = {id: 1, theme: current.themeval, font: 'Avenir-Next'}
    } else {
        value = {id: 1, theme: current.themeval, font: 'Avenir'}
    }

    await invoke('cmd_set_appstate', { conf: value })
    await getAppState()
}