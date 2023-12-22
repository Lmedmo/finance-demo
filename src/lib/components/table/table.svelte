<script lang="ts">
    import { TransactionsStore } from "$lib/controllers";
	import type { TableStore, TableFilter, Explorer, TableColumn } from "$lib/config";
    import { AppState, ExplorerState } from '$lib/config'
    import type { Transactions } from '$lib';
	import Rowendcap from './rowendcap.svelte';
	import Headings from './headings.svelte';
    import Selector from "./selector.svelte";
    import Thumbnail from "./thumbnail.svelte";
    import Cell from "./cell.svelte";

    export let storename: TableStore;
    export let thumbnails: TableStore;
    export let columns: TableColumn[];
    export let filter: TableFilter;

    let empty: boolean;
    let notype: boolean;

    async function getRecords(expl: Explorer, fil: TableFilter){
        console.log("fn call: getRecords")
        let dataset: Transactions | string | [];
        let store;
        let explrecs = [];
        let filtered = [];
        if (storename === "Transactions"){
            store = $TransactionsStore;
            notype = false
        } else {
            notype = true;
            dataset = { records: [], types: [], memos: []};
            return dataset
        }
        if (store.records.length > 0){
            empty = false;
            let recs;
            if (expl.selected.id == 0){
                recs = store.records
                //dataset = { fields: store.fields, records: store.records, types: store.types, memos: store.memos};
                //return dataset;
            } else {
                for(const record of store.records){
                    if (expl.references === 'Accounts' && expl.selected.id == record.account_id){
                        explrecs.push(record);
                    } else if (expl.references === 'Categories' && expl.selected.id == record.category_id){
                        explrecs.push(record);
                    }
                }
                recs = explrecs
                //dataset = { fields: store.fields, records: explrecs, types: store.types, memos: store.memos};
                //return dataset;
            }
            for(const item of recs){
                if (fil.fieldID == 'none'){
                    filtered.push(item)
                } else {
                    for(const value of fil.values){
                        if (item[fil.fieldID] === value){
                            filtered.push(item)
                        }
                    } 
                }
            }
            dataset = { records: filtered, types: store.types, memos: store.memos};
            return dataset;
        } else {
            empty = true;
            dataset = { records: [], types: [], memos: []};
            return dataset;
        }

    }
    
    $: style = `font-family: ${$AppState.font}; color: ${$AppState.text};`
</script>

<div class="Table">
{#await getRecords($ExplorerState, filter)}
    <p>Loading Data</p>
{:then data}
    {#if (empty)}
        <p>No data</p>
    {:else if (notype)}
        <p>Store doesn't exist</p>
    {:else}
        <div class="Tablename" {style}>{storename}</div>
        <table>
            <Headings {columns}/> 
            {#each data.records as item }
                <tr>
                    <Selector {item} />
                    <Thumbnail {item} ref={thumbnails}/>
                    {#each columns as col}
                        <Cell {item} {col}/>
                    {/each}
                    <Rowendcap />
                </tr>
            {/each}
        </table>
    {/if}
{/await}
</div>  
    
<style>
    .Table {
        display: flex;
        flex-direction: column;
        width: 100%;
        max-width: 1700px;
    }
    table {
        display: table;
        border-collapse: separate;
        border-spacing: 0px 4px;
        border-color: rgba(0, 0, 0, 0);
        width: 100%;
        /* padding-right: 25px; */
    }
    tr {
        display: table-row;
    }
    .Tablename {
        padding-left: 25px;
        padding-bottom: 10px;
        font-size: 1.7rem;
        font-weight: bold;
    }
</style>