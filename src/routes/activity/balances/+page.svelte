<script lang="ts">
	import { AccountsStore } from '$lib/controllers';
    import { Toolbar, Header, Table, Cluster, Viewlet, Calculation, Group, Typebutton, Actionbutton } from '$lib/components';
    import { ExplorerState, FormConfig, ReloadPending } from '$lib/config';
	import type { TableFilter, TableColumn } from '$lib/config';
    import { Content } from '$lib/layouts';
    
    function showImport(selectedID: number){
        let recs = $AccountsStore.records;
        let bankID = 1;
        let result = false;
        for(const rec of recs){
            if (rec.id == selectedID){
                bankID = rec.bank_id;
                break
            }
        }
        if (bankID > 1){
            result = true
        }
        return result
    }
    let filter: TableFilter = {table: "Transactions", fieldID: "none", values: [0]}
    let columns: TableColumn[] = [
        {fieldID: "type_id", headingname: "Type"},
        {fieldID: "memo_id", headingname: "Memo"},
        {fieldID: "date", headingname: "Date"},
        {fieldID: "merchant_id", headingname: "Merchant"},
        {fieldID: "depositor_id", headingname: "Depositor"},
        {fieldID: "category_id", headingname: "Category"},  
        {fieldID: "amount", headingname: "Amount"}
    ];
    $: selectedRecs = $FormConfig.subjects;
</script>

<Toolbar>
    {#if $AccountsStore.records.length > 0}
        <Group action={"Add"} ptype={"Transaction"}>
            <Typebutton stype={"Expense"} expl={$ExplorerState}/>
            <Typebutton stype={"Deposit"} expl={$ExplorerState}/>
            <Typebutton stype={"Transfer"} expl={$ExplorerState}/>
        </Group>
    {/if}

    {#if $ExplorerState.selected.id == 0}
        <Group action={"Add"} ptype={"Account"}>
            <Typebutton stype={"Credit Card"} expl={$ExplorerState}/>
            <Typebutton stype={"Mobile Account"} expl={$ExplorerState}/>
            <Typebutton stype={"Checking Account"} expl={$ExplorerState}/>
            <Typebutton stype={"Savings Account"} expl={$ExplorerState}/>
            <Typebutton stype={"Investment Account"} expl={$ExplorerState}/>
        </Group>
    {:else}
        <Actionbutton action={"Edit"} type={"Account"} expl={$ExplorerState}/>
    {/if}

    {#if showImport($ExplorerState.selected.id)}
        <Actionbutton action={"Import"} type={"Statement"} expl={$ExplorerState}/>
    {/if}

    {#if selectedRecs.length > 0}
        <div class="Divider"/>
        <Actionbutton action={"Delete"} type={"Transactions"} expl={$ExplorerState}/>
    {/if}
</Toolbar>

<Content>
    {#key $ReloadPending}
        <Header src={$ExplorerState.selected}/>
        <Cluster>
            {#if $ExplorerState.selected.id == 0}
                <Viewlet title={"Net Worth"}><Calculation fn={'Net-Worth'}/></Viewlet>
            {:else}
                <Viewlet title={"Current Balance"}><Calculation fn={'Balance'}/></Viewlet>
            {/if}
            <Viewlet title={"Upcoming"}></Viewlet>
        </Cluster>
        <Table storename="Transactions" thumbnails="Banks" {columns} {filter}/>
    {/key}
</Content>

<style>
    .Divider {
        box-sizing: border-box;
        width: 2px;
        height: 90%;
        background-color: #ffffff80;
        border-radius: 1px;
    }
</style>