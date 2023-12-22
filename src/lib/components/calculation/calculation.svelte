<script lang="ts">
    import type { Account, ExplOption } from "$lib";
    import { AccountsStore } from '$lib/controllers'
    import { ExplorerState, AppState } from '$lib/config'
	import { calcBalance, calcNetWorth, type Formulas } from "$lib/actions/calcs";
    import Dollar from "$lib/components/calculation/dollar.svelte";

    export let fn: Formulas;

    function getValue(accts: Account[], func: Formulas, expl: ExplOption){
        let val = 0.0
        let calcval = 0.0
        if (func === 'Net-Worth'){
            calcval = calcNetWorth(accts)
        } else if (func === 'Balance'){
            calcval = calcBalance(accts, expl)
        }
        val = Number((Math.round(calcval * 100) / 100).toFixed(2))
        return val
    }

    $: value = getValue($AccountsStore.records, fn, $ExplorerState.selected)
    $: style = `font-family: ${$AppState.font}; color: ${$AppState.text};`
</script>

{#if $AppState.os === 'darwin'}
    <div class="Calculation">
        <Dollar />
        <div {style} class="MacResult">{value}</div>
    </div>
{:else if $AppState.os === 'win32'}
    <div class="Calculation">
        <Dollar />
        <div {style} class="WinResult">{value}</div>
    </div>
{/if}

<style>
/* Common */
    div {
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .Calculation {
        flex-direction: row;
        gap: 8px;
    }
/* Mac */
    .MacResult {
        text-align: center;
        color: white;
        text-shadow: 4px 4px 8px rgba(0, 0, 0, 0.25);
        font-size: 4rem;
        font-weight: 500;
    }
/* Windows */
    .WinResult {
        text-align: center;
        color: white;
        text-shadow: 4px 4px 8px rgba(0, 0, 0, 0.25);
        font-size: 3.8rem;
        font-weight: 700;
    }
</style>