<script lang="ts">
    import { AppState } from '$lib/config';
    export let name: string;
    $: style = `background: ${$AppState.set_fill}`
    $: namestyle = `font-family: ${$AppState.font}; color: ${$AppState.text};`
</script>

<div {style} class="Setting">
    {#if $AppState.os === 'darwin'}
        <div style="{namestyle}" class="MacName">{name}</div>
        <slot></slot>
    {:else if $AppState.os === 'win32'}
        <div style="{namestyle}" class="WinName">{name}</div>
        <slot></slot>
    {/if}
</div>

<style>
    div {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
    }
    .Setting {
        border-radius: 12px;
        align-self: center;
        align-items: center;
        justify-content: space-between;
        border: 1px solid rgba(255, 255, 255, 0.25);
        box-shadow: 4px 4px 8px 0px rgba(0, 0, 0, 0.18), -2px -2px 2px 0px rgba(0, 0, 0, 0.15) inset;
        padding: 8px;
        width: 100%;
        max-width: 1700px;
    }
    .MacName {
        padding-left: 5px;
        font-size: 1.07rem;
        font-weight: 500; 
    }
    .WinName {
        padding-left: 5px;
        font-size: 0.9rem;
        font-weight: 600; 
    }
</style>