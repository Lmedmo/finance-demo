<script lang="ts">
	import { AppState, FormConfig } from '$lib';
    import { Buttons, Delete, Cancel } from '$lib/components/form/buttons';
    import Title from "../components/form/title.svelte";

    let recs: any = [];
    
    if ($FormConfig.rectype === "Transactions") {
        let subs = $FormConfig.subjects;
        for(const sub of subs){
            recs.push(sub)
        }
    }
    
    $: rt = $FormConfig.rectype
</script>

<div class="Delete-Form">
    <Title text={`Delete ${rt}`} />
    <h1 style="color: white; font-family: {$AppState.font};">Are you sure you want to delete the following records?</h1>
    <table>
        <thead>
        {#each Object.keys(recs[0]) as field}
            <th style="font-family: {$AppState.font}; color: {$AppState.color}">{field}</th>
        {/each}
        </thead>
        {#each recs as rec}
        <tr class="Row">
        {#each Object.values(rec) as val}
            <td class="Value" style="font-family: {$AppState.font};">{val}</td>
        {/each}
        </tr>
        {/each}   
    </table>
    
    <Buttons>
        <Cancel />
        <Delete />
    </Buttons>
</div>

<style>
    .Delete-Form {
        background-color: rgba(156, 156, 156, 0.6);
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        border-radius: 8px;
        backdrop-filter: blur(20px);
        box-shadow: 3px 3px 10px 1px rgba(0, 0, 0, 0.249);
        border: 2px solid #d1d1d140;
        align-items: center;
        justify-content: flex-start;
        align-self: stretch;
        padding-bottom: 10px;
        flex: 1 0;
        overflow: hidden;
        overflow-y: hidden;
    }
    table {
        display: table;
        box-sizing: border-box;
        justify-content: flex-start;
        background-color: #4d4d4d;
        border: 1px solid rgba(255, 255, 255, 0.3);
        border-radius: 5px;
        width: 70%;
        align-items: stretch;
        gap: 15px;
        overflow-y: auto;
    }
    thead{
        display: table-row;
        align-items: center;
        justify-content: start;
        text-align: left;
    }
    th {
        display: table-cell;
        justify-content: left;
        align-items: left;
        font-size: 1.1rem;
        font-weight: 600;
        text-align: left;
    }
    .Row {
        display: table-row;
    }
    .Value {
        display: table-cell;
        background: rgba(0, 0, 0, 0.5);
        padding: 5px 10px;
        color: white;
        font-size: 1rem;
        font-weight: 400;
    }
    h1 {
        width: 70%;
        font-size: 2.5rem;
        font-weight: 700;
    }
</style>