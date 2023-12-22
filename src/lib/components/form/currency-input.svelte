<script lang="ts">
	import { AppState } from "$lib/config";
    import { createEventDispatcher } from 'svelte';

    export let dollars: number | null = 0;
    export let cents: number | null = 0;

    let clicked = false

    $: dollars
    $: cents
    $: clicked
    $: dpl = dollars+''
    $: cpl = cents+''
    
    const dispatch = createEventDispatcher();

    function setAmount(){
        dispatch('amtSet')
    }
    
    function setNull(){
        dollars = null
        cents = null
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
<div class="CurrencyBox" {style} on:click|once={setNull}>
    <p {style}>$ </p>
    <input class="Currency" {style} type="number" placeholder={dpl} bind:value={dollars} on:change={setAmount}>
    <p {style}>.</p>
    <input class="Currency" {style} type="number" placeholder={cpl} bind:value={cents} on:change={setAmount}>
</div>

<style>
    .CurrencyBox {
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
    .Currency {
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