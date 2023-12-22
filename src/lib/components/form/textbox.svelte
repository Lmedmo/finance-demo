<script lang="ts">
	import { AppState } from "$lib/config";
	import { createEventDispatcher } from "svelte";

    export let text: string = '';

    $: text

    const dispatch = createEventDispatcher();

    function setText(){
        dispatch('textSet')
    }

    let style: string;

    $: if ($AppState.os === 'darwin'){
        style = `font-family: ${$AppState.font}; font-size: 1.1rem; font-weight: 400; color: ${$AppState.text};`;
    } else if ($AppState.os === 'win32'){
        style = `font-family: ${$AppState.font}; font-size: .9rem; font-weight: 500; color: ${$AppState.text};`;
    }
</script>

<input class="Textbox" type="text" {style} placeholder="" bind:value={text} on:change={setText}>

<style>
    .Textbox {
        width: 20%;
        appearance: none;
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        border-radius: 5px;
        border: none;        cursor: pointer;
        background-color: #ffffff7f;
        align-items: center;
        justify-content: center;
        text-align: center;
        vertical-align: middle;
        height: 30px;
        max-height: 30px;
    }
</style>