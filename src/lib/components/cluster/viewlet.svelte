<script lang="ts">
	import { AppState } from "$lib/config";

    export let title: string;

    let titleStyle: string;
    let cardStyle: string;

    $: titleStyle = `${$AppState.tb_txtclr}; font-family: ${$AppState.font};`
    $: cardStyle = `background: ${$AppState.viewlet};`;
</script>

<div class="Viewlet">
    {#if $AppState.os === 'darwin'}
        <div class="MacTitle" style="{titleStyle}">{title}</div>
        <div class="MacCard" style="{cardStyle}">
            <slot></slot>
        </div>
    {:else if $AppState.os === 'win32'}
        <div class="WinTitle" style="{titleStyle}">{title}</div>
        <div class="WinCard" style="{cardStyle}">
            <slot></slot>
        </div> 
    {/if}
    
</div>
<style>
    .Viewlet {
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        padding-bottom: 25px;
        flex: 1 0;
        gap: 10px;
    }
/* Mac */
    .MacTitle {
        font-size: 1.7rem;
        font-weight: 700;
    }
    .MacCard {
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        align-items: center;
        justify-content: center;
        flex: 1 0 0;
        border-radius: 15px;
        border: 1px solid rgba(255, 255, 255, 0.25);
        box-shadow: 4px 4px 8px 0px rgba(0, 0, 0, 0.18), -2px -2px 2px 0px rgba(0, 0, 0, 0.15) inset;
        padding: 10px;
    }
/* Windows */
    .WinTitle {
        font-size: 1.4rem;
        font-weight: bold;
    }
    .WinCard {
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        align-items: center;
        justify-content: center;
        flex: 1 0 0;
        border-radius: 10px;
        border: 1px solid rgba(255, 255, 255, 0.25);
        box-shadow: 4px 4px 8px 0px rgba(0, 0, 0, 0.18), -2px -2px 2px 0px rgba(0, 0, 0, 0.15) inset;
        padding: 10px;
    }
</style>