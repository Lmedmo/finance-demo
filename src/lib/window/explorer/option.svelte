<script lang="ts">
	import type { ExplOption } from "$lib/config";
	import { AppState, ExplorerState, FormState } from "$lib/config";

    export let option: ExplOption;

    let style: any;
    let btnStyle: string;
    let hoverStyle: string;
    let txtstyle: string;
    let iconStyle: string;
    let hovered: boolean;
    let disabled: boolean;

    $: disabled = !$FormState

    function setSelected(opt: ExplOption, active: boolean) {
        if (active){
            let expl = $ExplorerState
            expl.selected = opt;
            ExplorerState.set(expl)
        }
        console.log("Explorer Option set to: ", opt)
    }
    
    $: if ($ExplorerState.selected.id == option.id) {
            btnStyle = `background-color: ${$AppState.rib_expl_btn}; box-shadow: 2px 2px 5px 0px #00000026, -3px -3px 5px 0px #0000001a inset;`;
            hoverStyle = btnStyle;
            txtstyle = `color: ${$AppState.color}; font-family: ${$AppState.font};`;
            iconStyle = `background: ${option.icon_color}; box-shadow: 0px 0px 3px 3px rgba(0, 219, 153, 0.70);`;
        } else {
            btnStyle = `background-color: #00000000;`;
            hoverStyle = `background-color: #00000015;`;
            txtstyle = `color: ${$AppState.text}; text-shadow: 2px 2px 5px rgba(0, 0, 0, 0.20); font-family: ${$AppState.font};`;
            iconStyle = `background: ${option.icon_color};`;
        }
    $: if (hovered) {style = hoverStyle} else {style = btnStyle};
</script>

{#if $AppState.os === 'darwin'}
    <button class="MacOption" 
        {style}
        on:click="{() => setSelected(option, disabled)}" 
        on:mouseenter={() => {hovered = true}} 
        on:mouseleave={() => {hovered = false}}
    >
        <img class="Macimg" src="{option.icon}" alt="account icon" style={iconStyle}>
        {#if $AppState.explorer}
            <div class="MacName" style={txtstyle}>{option.name}</div>
        {/if}
    </button>
{:else if $AppState.os === 'win32'}
    <button class="WinOption" 
        {style}
        on:click="{() => setSelected(option, disabled)}" 
        on:mouseenter={() => {hovered = true}} 
        on:mouseleave={() => {hovered = false}}
    >
        <img class="Winimg" src="{option.icon}" style={iconStyle} alt="account icon">
        {#if $AppState.explorer}
            <div class="WinName" style={txtstyle}>{option.name}</div>            
        {/if}
    </button> 
{/if}

{#if option.id == 0}
    <div class="Divider"/>
{/if}

<style>
/* Common */
    img {
        display: flex;
        box-sizing: border-box;
        align-items: center;
        justify-content: center;
        /* background-color: none; */
    }
    .Divider {
        flex-direction: row;
        background: #4d4d4d20;
        height: 2px;
        width: 100%;
        margin: 2px;
        justify-content: center;
        align-self: stretch;
    }
/* Mac */
    .MacOption {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        align-items: center;
        align-self: stretch;
        padding: 8px;
        gap: 15px;
        border-radius: 12px;
        border: none;
        cursor: pointer;
    }
    .Macimg {
        border-radius: 8px;
        width: 35px;
        height: 35px;
    }
    .MacName {
        display: flex;
        box-sizing: border-box;
        height: 35px;
        padding-right: 30px;
        align-items: center;
        font-size: 1.07rem;
        font-weight: 500;
    }
/* Windows */
    .WinOption {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        align-items: center;
        align-self: stretch;
        padding: 6px;
        gap: 15px;
        border-radius: 9px;
        border: none;
        cursor: pointer;
    }
    .Winimg {
        border-radius: 5px;
        width: 25px;
        height: 25px;
    }
    .WinName {
        display: flex;
        box-sizing: border-box;
        height: 25px;
        padding-right: 30px;
        align-items: center;
        font-size: .8rem;
        font-weight: 600;
    }
</style>