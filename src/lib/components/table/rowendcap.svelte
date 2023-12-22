<script lang="ts">
	import { AppState } from "$lib/config";

    let style: string;

    $: if ($AppState.theme === 'light'){
        style = `background: #bebdbf4d; text-shadow: 0px 0px 4px #ffffffb3; border-top: 1px solid #00000026; border-bottom: 1px solid #00000026; border-right: 1px solid rgb(0, 0, 0, 15%);`
    } else {
        style = `background: #ffffff66; text-shadow: 0px 0px 4px #00000033; border-top: 1px solid #ffffff26; border-bottom: 1px solid #ffffff26; border-right: 1px solid rgb(255, 255, 255, 15%);`
    }
</script>

{#if $AppState.os === 'darwin'}
    <td class="MacRowcap" {style}/>
{:else if $AppState.os === 'win32'}
    <td class="WinRowcap" {style}/>
{/if}

<style>
    td {
        display: table-cell;
        width: 9px;
        min-width: 9px;
        align-items: flex-start;
        border-radius: 0px 8px 8px 0px;
        box-sizing: border-box;
    }
    .MacRowcap {
        height: 35px;
    }
    .WinRowcap {
        height: 30px;
    }
</style>