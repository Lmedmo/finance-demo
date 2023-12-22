<script lang="ts">
    import { AppState } from "$lib/config";
    
    export let value: string;

    $: style = `font-family: ${$AppState.font}; color: ${$AppState.text};`
</script>

{#if $AppState.os === 'darwin'}
    <div class="MacHeading" {style}>{value}</div>
{:else if $AppState.os === 'win32'}
    <div class="WinHeading" {style}>{value}</div>
{/if}

<style>
    div {
        display: flex;
        box-sizing: border-box;
        justify-content: flex-start;
        margin-top: 25px;
        margin-bottom: 3px;
        width: 100%;
        max-width: 1700px;
        align-self: center;
    }
    .MacHeading {
        /* background-color: aqua; */
        font-size: 1.6rem;
        font-weight: 700;
    }
    .WinHeading {
        /* background-color: aqua; */
        font-size: 1.4rem;
        font-weight: 700;
    }
</style>