<script lang="ts">
	import { PageState, AppState, FormState } from "$lib";
	import type { Page } from '$lib/config';


    export let page: Page;

    let style: string;
    let blendL: string;
    let blendR: string;

    function setSelected(selectedpage: Page) {
        console.log("Page Selected: ", selectedpage)
        PageState.set(selectedpage)
    }
    
    $: if ($PageState.id === page.id) {
        style = `background-color: ${$AppState.color}; color: white; font-family: ${$AppState.font};`;
        blendL = `box-shadow: 6px 0px ${$AppState.color}; border-bottom-right-radius: 6px;`;
        blendR = `box-shadow: 0px 6px ${$AppState.color}; border-bottom-Left-radius: 6px;`;
    } else {
        style = `color: ${$AppState.text}; font-family: ${$AppState.font};`;
        blendL = `box-shadow: 6px 0px #00000000; border-bottom-right-radius: 6px;`;
        blendR = `box-shadow: 0px 6px #00000000; border-bottom-Left-radius: 6px;`;
    }
</script>

{#if $AppState.os === 'darwin'}
    <div data-tauri-drag-region class="Tab" style="height: 30px;">
        <div class="Blend" style="{blendL}"/>
        {#if $FormState}
            <button class="Macbutton" {style}>{page.title}</button>
        {:else}
            <button class="Macbutton" {style} on:click="{ () => setSelected(page) }">{page.title}</button>
        {/if}
        <div class="Blend" style="{blendR}"/>
    </div>
{:else if $AppState.os === 'win32'}
    <div data-tauri-drag-region class="Tab" style="height: 25px;">
        <div class="Blend" style="{blendL}"/>
        {#if $FormState}
            <button class="Winbutton" {style}>{page.title}</button>
        {:else}
            <button class="Winbutton" {style} on:click="{ () => setSelected(page) }">{page.title}</button>
        {/if}
        <div class="Blend" style="{blendR}"/>
    </div>
{/if}

<style>
    .Tab {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        align-items: flex-end;
        justify-content: space-around;
    }
    .Macbutton {
        height: 30px;
        font-size: 1.2rem;
        font-weight: 400;
        padding: 0px 10px 5px 10px;
    }
    .Winbutton {
        height: 25px;
        font-size: .9rem;
        font-weight: 600;
        padding: 0px 7px 2.5px 7px;
    }
    button {
        /* background-color: rgb(255, 0, 0); */
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        cursor: pointer;
        justify-content: center;
        align-items: center;
        border: none;
        border-radius: 6px 6px 0px 0px;
        text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.15);
        background-color: #00000000;
    }
    .Blend {
        background-color: #00000000;
        display: flex;
        box-sizing: border-box;
        height: 12px;
        width: 12px;
        align-items: flex-end;
        justify-content: flex-end;
    }
</style>