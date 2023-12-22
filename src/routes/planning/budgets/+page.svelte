<script lang="ts">
	import { Content } from '$lib/layouts';
    import { Header, Toolbar, Actionbutton, Table } from '$lib/components';
	import { ExplorerState } from '$lib/config';
	import { BudgetsStore } from '$lib';
</script>

<Toolbar>
    {#if $ExplorerState.selected.id != 0}
        <Actionbutton action={"Edit"} type={"Budget"} expl={$ExplorerState}/>
    {/if}
    <Actionbutton action={"Add"} type={"Budget"} expl={$ExplorerState}/>
</Toolbar>

<Content>
    <!-- <Header icon={$ExplorerState.selected}/> -->
    <!-- <Table storename="Transactions" exploreRef="Categories" thumbnails="Merchants"/> -->
    {#each $BudgetsStore.records as rec}
        <p>Budget ID: {rec.id}; Month: {rec.month}; Year: {rec.year}</p>
        <p>Categories</p>
        {#each rec.categories as cat, i}
            <p>{i}. BudgetID: {cat.budget_id}; CategoryID: {cat.category_id}; CalcType: {cat.calc_type}; Name?: {cat.name}; Total: ${cat.total}</p>
        {/each}
    {/each}
</Content>