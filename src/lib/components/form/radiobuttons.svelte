<script lang="ts">
	import type { Account, PeriodUnitType } from "$lib";
	import { AppState, FormConfig, setCollection } from "$lib/config";

    export let store: PeriodUnitType[];
    export let fcindex: string; /* FormConfig.Collection attribute name (ex. period_unit, t_type, etc.) */
    export let srindex: string; /* Options Store record attribute name (ex. name, icon, etc.)*/

    $: fc = $FormConfig.collection as Account;

    function setSelected(sr: PeriodUnitType){
        fc[fcindex] = sr[srindex]
        setCollection(fc)
    };

    let style = '';
    let sel_style = '';
    
    $: if ($AppState.os === 'darwin'){
        style = `background-color: ${$AppState.fill}; border: 2px solid ${$AppState.color}; color: ${$AppState.color}; font-family: ${$AppState.font}; font-size: 1.1rem; font-weight: 400;`
        sel_style = `background-color: ${$AppState.color}; border: 2px solid ${$AppState.color}; color: ${$AppState.text}; font-family: ${$AppState.font}; font-size: 1.1rem; font-weight: 400;`
    } else {
        style = `background-color: ${$AppState.fill}; border: 2px solid ${$AppState.color}; color: ${$AppState.color}; font-family: ${$AppState.font}; font-size: 1.1rem; font-weight: 400;`
        sel_style = `background-color: ${$AppState.color}; border: 2px solid ${$AppState.color}; color: ${$AppState.text}; font-family: ${$AppState.font}; font-size: .95rem; font-weight: 400;`
    }
</script>

<div class="Group">
    {#each store as rec}
        {#if fc[fcindex] === rec[srindex]}
            <button class="Radio" style={sel_style} on:click={()=>setSelected(rec)}>{rec[srindex]}</button>
        {:else}
            <button class="Radio" {style} on:click={()=>setSelected(rec)}>{rec[srindex]}</button>
        {/if}
    {/each}
</div>

<style>
    .Group {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        border-radius: 5px;
        gap: 5px;
        background-color: none;
        align-items: center;
        justify-content: center;
    }
    .Radio {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        justify-content: center;
        align-items: center;
        padding: 3px 7px;
        background-color: #ffffff4d;
        border-radius: 6px;
        border: none;
        cursor: pointer;
    }
</style>