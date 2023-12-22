<script lang="ts">
	import { AppState, FormConfig, FormState } from '$lib/config';
	import type { RecordType, ToolbarAction } from '$lib/config';
    
    export let action: ToolbarAction;
    export let ptype: RecordType;

    const fc = $FormConfig
    let expanded: boolean;
    $: expanded

    function handleClick(){
        if (action === "Add"){
            fc.formtype = "Entry"
        } else if (action === "Delete"){
            fc.formtype = "Delete"
        } else if (action === "Edit"){
            fc.formtype = "Edit"
        }
        FormConfig.set(fc)
        hovered = true
        expanded = true
        document.body.addEventListener('click', collapse)
    }
    function collapse(){
        hovered = false
        expanded = false
        document.body.removeEventListener('click', collapse)
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
        <button class="MacButton" {style}>{action} {ptype}</button>
    {:else}
    <div class="Group">
        {#if expanded}
            <button class="MacButton" {style} on:click|stopPropagation={handleClick}>{action} {ptype}</button>
            <div class="Drawer">
                <slot></slot>
            </div>
        {:else}
            <button class="MacButton" {style} on:click|stopPropagation={handleClick} on:mouseenter={hover} on:mouseleave={unhover}>{action} {ptype}</button>
        {/if}
    </div>
    {/if}
{:else if $AppState.os === 'win32'}
    {#if $FormState}
        <button class="WinButton" {style}>{action} {ptype}</button>
    {:else}
    <div class="Group">
        {#if expanded}
            <button class="WinButton" {style} on:click|stopPropagation={handleClick}>{action} {ptype}</button>
            <div class="Drawer">
                <slot></slot>
            </div>
        {:else}
            <button class="WinButton" {style} on:click|stopPropagation={handleClick} on:mouseenter={hover} on:mouseleave={unhover}>{action} {ptype}</button>
        {/if}
    </div>
    {/if}
{/if}

<style>
    .Group {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }
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
    .Drawer {
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        padding: 5px;
        gap: 5px;
        border-radius: 8px;
        align-items: stretch;
        justify-content: center;
        border: 1px solid #4d4d4d40;
        background: #00000080;
        box-shadow: 0px 2px 4px 0px #00000036;
        backdrop-filter: blur(2px);
    }
</style>