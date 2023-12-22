<script lang="ts">
	import { type View, ViewState, AppState, FormState } from "$lib";


    export let view: View;

    let style: string;

    function setSelected(selectedview: View) {
        console.log("View Selected: ", selectedview)
        ViewState.set(selectedview)
    }

    $: if ($ViewState.id === view.id) {
        style = `background-color: ${$AppState.rib_expl_btn}; box-shadow: 2px 2px 5px 0px #00000026, -2px -2px 3px 0px #0000001a inset; color: ${$AppState.color}; font-family: ${$AppState.font};`;
    } else {
        style = `background-color: #00000000; color: white; text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.15); font-family: ${$AppState.font};`;
    }
</script>

{#if $AppState.os === 'darwin'}
    {#if $FormState}
        <button class="MacButton" {style}>{view.title}</button> 
    {:else}
        <button class="MacButton" {style} on:click="{ () => setSelected(view) }">{view.title}</button> 
    {/if}
{:else if $AppState.os === 'win32'}
    {#if $FormState}
        <button class="WinButton" {style}>{view.title}</button>           
    {:else}
        <button class="WinButton" {style} on:click="{ () => setSelected(view) }">{view.title}</button>           
    {/if}
{/if}

<style>
    button {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        cursor: pointer;
        justify-content: center;
        align-items: center;
        border: none;
    }
    .MacButton {
        /* background-color: #00000000; */
        padding: 8px 9px;
        border-radius: 7px;
        font-size: 1.2rem;
        font-weight: 500;
    }
    .WinButton {
        /* background-color: #00000000; */
        padding: 6px 7px;
        border-radius: 5px;
        font-size: .9rem;
        font-weight: 600;
    }
</style>