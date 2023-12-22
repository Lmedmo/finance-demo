<script lang="ts">
	import { AppState } from "$lib/config";
    import { createEventDispatcher } from 'svelte';

    export let rate: number | null = 0;
    
    $: rate

    const dispatch = createEventDispatcher();

    function setRate(){
        dispatch('rateSet')
    }
    function setNull(){
        rate = null
    }

    let style: string;

    $: if ($AppState.os === 'darwin'){
        style = `font-family: ${$AppState.font}; font-size: 1.1rem; font-weight: 400; color: ${$AppState.text};`;
    } else if ($AppState.os === 'win32'){
        style = `font-family: ${$AppState.font}; font-size: .9rem; font-weight: 500; color: ${$AppState.text};`;
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="PercentageBox" {style} on:click|once={setNull}>
    <input class="Percentage" {style} type="number" placeholder="0" bind:value={rate} on:change={setRate}>
    <p {style}>%</p>
</div>

<style>
    .PercentageBox {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        border: none;
        cursor: pointer;
        text-align: center;
        vertical-align: middle;
        width: 20%;
        max-width: 20%;
        height: 30px;
        max-height: 30px;
    }
    .Percentage {
        width: 100%;
        appearance: none;
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        border-radius: 5px;
        border: none;
        cursor: pointer;
        background-color: #ffffff7f;
        text-align: center;
        vertical-align: middle;
        height: 30px;
        max-height: 30px;
    }
    p {
        display: flex;
        box-sizing: border-box;
        align-self: center;
        flex: 1 0;
        text-align: center;
        vertical-align: middle;
    }
</style>