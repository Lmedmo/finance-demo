<script lang="ts">
	import { AppState } from "$lib/config";
	import { createEventDispatcher } from "svelte";

    export let date: string;

    $: date
    //$: dpl = Date.parse(date) 

    const dispatch = createEventDispatcher();

    function setDate(){
        dispatch('dateSet')
        console.log("date", date)
    }

    let style: string;

    $: if ($AppState.os === 'darwin'){
        style = `font-family: ${$AppState.font}; font-size: 1.1rem; font-weight: 500; color: ${$AppState.text};`;
    } else if ($AppState.os === 'win32'){
        style = `font-family: ${$AppState.font}; font-size: .9rem; font-weight: 500; color: ${$AppState.text};`;
    }
</script>

<!-- <input class="Textbox" type="text" {style} placeholder="" bind:value={text} on:change={setText}> -->
<input class="Datepicker" type="date" {style} placeholder={date} bind:value={date} on:change={setDate}>


<style>
    .Datepicker {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        height: 30px;
        max-height: 30px;
        width: 20%;
        border-radius: 5px;
        border: none;
        border-color: #ffffffff;
        cursor: pointer;
        background: #ffffff7f;
        align-items: center;
        justify-content: center;
        text-align: center;
        vertical-align: middle;

        text-shadow: none;
        box-shadow: none;
        text-emphasis: none;
        text-decoration: none;
        outline: none;
        opacity: 1;
        color: white;
        background-blend-mode: normal;
        appearance: none;
        accent-color: #00000000;
        filter: contrast(200%);
        mix-blend-mode: normal;
    }
    .Datepicker::placeholder{
        text-shadow: none;
        box-shadow: none;
        text-emphasis: none;
        text-decoration: none;
        outline: none;
        opacity: 1;
        color: white;
        background-blend-mode: normal;
        appearance: none;
        accent-color: #00000000;
        -webkit-text-fill-color: white;
        filter: contrast(200%);
        mix-blend-mode: normal;
    }
    .Datepicker:-moz-placeholder{
        text-shadow: none;
        box-shadow: none;
        text-emphasis: none;
        text-decoration: none;
        outline: none;
        opacity: 1;
        color: white;
        background-blend-mode: normal;
        appearance: none;
        accent-color: #00000000;
        -webkit-text-fill-color: white;
        filter: contrast(200%);
        mix-blend-mode: normal;
    }
    .Datepicker:-webkit-input-placeholder{
        text-shadow: none;
        box-shadow: none;
        text-emphasis: none;
        text-decoration: none;
        outline: none;
        opacity: 1;
        color: white;
        background-blend-mode: normal;
        appearance: none;
        accent-color: #00000000;
        -webkit-text-fill-color: white;
        filter: contrast(200%);
        mix-blend-mode: normal;
    }
</style>