<script lang="ts">
	import { toggleSelected, type TableColumn, type Transaction, ExplorerState, AppState, FormState } from "$lib";
	import { initEditForm } from "$lib/actions";
    
    export let item: Transaction;
    export let col: TableColumn;

    let value = getValue(col.fieldID, item)
    let style: string;
    
    function getValue(colname: string, rec: Transaction){
        let value;
        if (colname === "merchant_id"){
            value = rec.merchant_name;
        } else if (colname === "date"){
            value = rec.date;
        } else if (colname === "description"){
            value = rec.description;
        } else if (colname === "category_id"){
            value = rec.category_name
        } else if (colname === "amount"){
            let amt = Number(item.amount)
            value = "$" + (Math.round(amt * 100) / 100).toFixed(2);
        } else if (colname === "memo_id"){
            value = rec.memo_name
        } else if (colname === "goal_id"){
            value = rec.goal_name
        } else if (colname === "depositor_id"){
            value = rec.depositor_name
        } else if (colname === "to_from_account"){
            value = rec.to_from_acct_name
        } else if (colname === "account_id"){
            value = rec.account
        } else if (colname === "id"){
            value = rec.id
        } else if (colname === "type_id"){
            value = rec.type_name
        }
        if (typeof value === 'undefined'){
            console.log(`getValue returning undefined: colname = ${colname}; rec: `, rec)
            return 'none'
        } else {
            return value
        }
    }
    function editItem(record: Transaction){
        toggleSelected(record);
        if (1 < record.memo_id && record.memo_id < 5){
            initEditForm("Expense", $ExplorerState)
        } else if (record.memo_id < 8){
            initEditForm("Deposit", $ExplorerState)
        } else {
            initEditForm("Transfer", $ExplorerState)
        }
    }
    $: if ($AppState.theme == 'light'){
        if (typeof value === 'undefined' || value === 'Unrecognized'){
            style = `background: #bebdbf4d; text-shadow: 0px 0px 4px #ffffffb3; border-top: 1px solid #00000026; border-bottom: 1px solid #00000026; font-family: ${$AppState.font}; color: #FFD600;`
        } else if (col.fieldID === 'amount' && item.type_id == 2){
            style = `background: #bebdbf4d; text-shadow: 0px 0px 4px #ffffffb3; border-top: 1px solid #00000026; border-bottom: 1px solid #00000026; font-family: ${$AppState.font}; color: #44D201; font-weight: 500;`
        } else if (col.fieldID === 'amount' && item.type_id == 3){
            style = `background: #bebdbf4d; text-shadow: 0px 0px 4px #ffffffb3; border-top: 1px solid #00000026; border-bottom: 1px solid #00000026; font-family: ${$AppState.font}; color: #FF0000; font-weight: 500;`;
        } else {
            style = `background: #bebdbf4d; text-shadow: 0px 0px 4px #ffffffb3; border-top: 1px solid #00000026; border-bottom: 1px solid #00000026; font-family: ${$AppState.font}; color: ${$AppState.text};`
        }
    } else {
        if (typeof value === 'undefined' || value === 'Unrecognized'){
            style = `background: #ffffff66; text-shadow: 0px 0px 4px #00000033; border-top: 1px solid #ffffff26; border-bottom: 1px solid #ffffff26; font-family: ${$AppState.font}; color: #FFD600;`
        } else if (col.fieldID === 'amount' && item.type_id == 2){
            style = `background: #ffffff66; text-shadow: 0px 0px 4px #00000033; border-top: 1px solid #ffffff26; border-bottom: 1px solid #ffffff26; font-family: ${$AppState.font}; color: #44D201; font-weight: 500;`
        } else if (col.fieldID === 'amount' && item.type_id == 3){
            style = `background: #ffffff66; text-shadow: 0px 0px 4px #00000033; border-top: 1px solid #ffffff26; border-bottom: 1px solid #ffffff26; font-family: ${$AppState.font}; color: #dd0c0f; font-weight: 500;`;
        } else {
            style = `background: #ffffff66; text-shadow: 0px 0px 4px #00000033; border-top: 1px solid #ffffff26; border-bottom: 1px solid #ffffff26; font-family: ${$AppState.font}; color: ${$AppState.text};`
        }
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
{#if $AppState.os === 'darwin'}
    {#if $FormState}
        <td class="MacCell" {style}>
            {value}
        </td>
    {:else}
        <td class="MacCell" {style} on:click={()=>{editItem(item)}}>
            {value}
        </td>
    {/if}    
{:else if $AppState.os === 'win32'}
    {#if $FormState}
        <td class="WinCell" {style}>
            {value}
        </td>
    {:else}
        <td class="WinCell" {style} on:click={()=>{editItem(item)}}>
            {value}
        </td>
    {/if} 
{/if}

<style>
    td {
        display: table-cell;
        box-sizing: border-box;
        cursor: pointer;
        max-width: 500px;
        white-space: nowrap; 
        overflow: hidden;
        text-overflow: ellipsis;
        direction: ltr;
        padding: 5px 20px 5px 0px;
        vertical-align: middle;
    }
    .MacCell {
        height: 35px;
        font-size: 1rem;
    }
    .WinCell {
        height: 30px;
        font-size: .8rem;
    }
</style>