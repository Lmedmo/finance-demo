<script lang="ts">
    import { AppState, changeFont, changeTheme } from "$lib/config";
   
    export let value: string;
    export let state: string;
    export let context: string;

    function selectVal(val: string, ctx: string){
        if (ctx === 'Theme'){
            changeTheme(val, $AppState)
        } else if (ctx === 'Font'){
            changeFont(val, $AppState)
        }
    }
   
    let style: string;
    
    $: if (value === state){
        style = `background-color: ${$AppState.color}; font-family: ${$AppState.font}; color: ${$AppState.text};`;
    } else {
        style = `font-family: ${$AppState.font}; color: ${$AppState.text};`;
    }
   </script>
   
   {#if $AppState.os === 'darwin'}
       <button class="MacValue" {style} on:click={()=>{selectVal(value, context)}}>{value}</button>
   {:else if $AppState.os === 'win32'}
       <button class="WinValue" {style} on:click={()=>{selectVal(value, context)}}>{value}</button>
   {/if}
   
<style>
    .MacValue {
        background-color: rgba(255, 255, 255, 0.3);
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        justify-content: center;
        align-items: center;
        padding: 7px 12px;
        border: 1px solid rgba(255, 255, 255, 0.25);
        border-radius: 8px;
        cursor: pointer;

        font-size: 1.07rem;
        font-weight: 500;
        text-transform: capitalize;
    }
    .WinValue {
        background-color: rgba(255, 255, 255, 0.3);
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        justify-content: center;
        align-items: center;
        padding: 7px 12px;
        border: 1px solid rgba(255, 255, 255, 0.25);
        border-radius: 8px;
        cursor: pointer;
        
        font-size: 0.9rem;
        font-weight: 500;
        text-transform: capitalize;
    }
</style>