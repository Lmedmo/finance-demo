<script lang="ts">
	import Option from './option.svelte';
    import Divider from './divider.svelte';
	import { AppState, PageState, ViewState, fetchExplorer, getOpts, type Explorer, type ExplOption } from '$lib/config';
	import { AccountsStore, CategoriesStore, DepositorStore, BudgetsStore, MerchantsStore } from '$lib';
    
    function getExplOpts(expl: Explorer){
        let options: ExplOption[] = [];
        let ref = expl.references
        
        if (ref === "Accounts"){
            options = getOpts("Account", expl.selected, $AccountsStore)
        } else if (ref === "Categories"){
            options = getOpts("Category", expl.selected, $CategoriesStore)
        } else if (ref === "Depositors"){
            options = getOpts("Depositor", expl.selected, $DepositorStore)
        } else if (ref === "Budgets"){
            options = getOpts("Budget", expl.selected, $BudgetsStore)
        } else if (ref === "Goals"){
            options = getOpts("Goal", expl.selected, $BudgetsStore)
        } else if (ref === "Merchants"){
            options = getOpts("Merchant", expl.selected, $MerchantsStore)
        } else {
            options = []
        }
        return options
    }

    $: style = `${$AppState.expl_fill}`;
</script>

{#await fetchExplorer($PageState, $ViewState) then explorer}
    {#if explorer.shown}
    <div class="Explorer" >
        <div {style} class="Pane">
            <div class="Options">
            {#each getExplOpts(explorer) as option}
            <Option {option} />
            {/each}
            </div>
        </div>
        <Divider />
    </div>
    {/if}
{/await}

<style>
    .Explorer {
        display: flex;
        flex-direction: row;
        box-sizing: border-box; 
        align-items: center;
        align-self: stretch;
        /* width: 15%; */
        max-width: 400px;
        min-width: fit-content;
        resize: horizontal;
    }
    .Pane {
        display: flex;
        box-sizing: border-box; 
        padding: 10px;
        border-radius: 8px;
        box-shadow: 1px 1px 3px 0px #0000001a;
        backdrop-filter: blur(10px);
        align-items: center;
        align-self: stretch;
        flex: 1 0;
        min-width: fit-content;
    }
    .Options {
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        align-items: center;
        align-self: stretch;
        align-items: flex-start;
        gap: 12px;
        /* flex: 1 0; */
    }
</style>