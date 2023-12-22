<script lang="ts">
    import { AppState } from "$lib/config";

    export let value: string | number | undefined;
    
    $: style = `font-family: ${$AppState.font}; color: ${$AppState.text};`;
</script>

{#if $AppState.os === 'darwin'}
    <div {style} class="MacValue">{value}</div>
{:else if $AppState.os === 'win32'}
    <div {style} class="WinValue">{value}</div>
{/if}

<style>
    div {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        background-color: rgba(255, 255, 255, 0.3);
        border: 1px solid rgba(255, 255, 255, 0.25);
        border-radius: 8px;
    }
    .MacValue {
        padding: 8px;
        font-size: 1.07rem;
        font-weight: 500;
    }
    .WinValue {
        padding: 5px;
        font-size: 0.9rem;
        font-weight: 500;
    }
</style>