<script lang="ts">
	import { AppState } from "$lib/config";
	import { createEventDispatcher } from "svelte";

    export let store: any[];
    export let selected: any;

    let clicked = false
    $: selected
    $: clicked
    
    const dispatch = createEventDispatcher();

    function setSelected(){
        dispatch('optSet')
    }
    function removePrefill(){
        clicked = true
    }
    function getOptions(recs: any){
        const opts = []
        for (const rec of recs){
            let opt = {id: rec.id, name: rec.name}
            opts.push(opt)
        }
        return opts
    }

    let style: string;
    let ostyle: string;
    $: if ($AppState.os === 'darwin'){
        ostyle = `font-family: ${$AppState.font}; font-size: 1.1rem; color: ${$AppState.text};`;
        style = `font-family: ${$AppState.font}; font-size: 1.1rem; color: ${$AppState.text};`;
    } else if ($AppState.os === 'win32'){
        ostyle = `font-family: ${$AppState.font};`;
        style = `font-family: ${$AppState.font};`;
    }
</script>

{#if clicked}
    <select class="Selector" {style} bind:value={selected} on:change={setSelected}>
        {#each getOptions(store) as rec}
            <option class="Option" style="{ostyle}" value="{rec}">{rec.name}</option>
        {/each}
    </select>
{:else}
    <select class="Selector" {style} bind:value={selected} on:change={setSelected} on:click={removePrefill}>
        {#each store as rec}
            {#if rec.id == selected.id}
            <option class="Option" style="{ostyle}" value="{selected}" selected>{selected.name}</option>
            {:else}
            <option class="Option" style="{ostyle}" value="{rec}">{rec.name}</option>
            {/if}
        {/each}
    </select>
{/if}

<style>
    .Selector {
        appearance: none;
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        width: 20%;
        border-radius: 5px;
        border: none;
        padding: 5px;
        cursor: pointer;
        background-color: #ffffff7f;
        align-items: center;
        justify-content: center;
        text-align: center;
        vertical-align: middle;
    }
    .Option {
        appearance: none;
        display: flex;
        box-sizing: border-box;
        font-weight: 600;
    }
</style>