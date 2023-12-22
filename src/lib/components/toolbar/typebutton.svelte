<script lang="ts">
	import { AppState, FormState, FormConfig } from '$lib/config';
	import type { Explorer, RecordType } from '$lib/config';
    import { initAddForm, initDeleteForm, initEditForm } from '$lib/actions';
    
    export let stype: RecordType;
    export let expl: Explorer;

    let action = $FormConfig.formtype

    function handleClick(){
        if (action === "Entry"){
            initAddForm(stype, expl)
        } else if (action === "Delete"){
            initDeleteForm(stype, expl)
        } else if (action === "Edit"){
            initEditForm(stype, expl)
        }
        hovered = false
    }

    let hovered: boolean;
    let style: string;

    const hover = () => { hovered = true }
    const unhover = () => { hovered = false }

    $: btnStyle = `${$AppState.tool_btn}; color: white; font-family: ${$AppState.font};`;
    $: hoverStyle = `background: ${$AppState.color}; color: white; font-family: ${$AppState.font};`;

    $: if (hovered) {style = hoverStyle;} else {style = btnStyle};
</script>

{#if $AppState.os === 'darwin'}
    {#if $FormState}
        <button class="MacButton" {style}>{stype}</button>
    {:else}
        <button class="MacButton" {style} on:click={handleClick} on:mouseenter={hover} on:mouseleave={unhover}>{stype}</button>
    {/if}
{:else if $AppState.os === 'win32'}
    {#if $FormState}
        <button class="WinButton" {style}>{stype}</button>
    {:else}
        <button class="WinButton" {style} on:click={handleClick} on:mouseenter={hover} on:mouseleave={unhover}>{stype}</button>
    {/if}
{/if}

<style>
    .MacButton {
        border-radius: 6px;
        border: none;
        padding: 4px 9px;
        text-decoration: none;
        cursor: pointer;
        box-shadow: 2px 2px 5px 0px #00000026, -2px -2px 2px 0px #00000014 inset;
        font-weight: 500;
        text-align: center;
        font-size: .9rem;
    }
    .WinButton {
        border-radius: 5px;
        border: none;
        padding: 4px 9px;
        text-decoration: none;
        cursor: pointer;
        box-shadow: 2px 2px 5px 0px #00000026, -2px -2px 2px 0px #00000014 inset;
        font-weight: 600;
        text-align: center;
        font-size: .7rem;
    }
</style>