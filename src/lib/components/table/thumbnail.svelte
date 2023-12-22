<script lang="ts">
    import type { Bank, Merchant, TableStore, Transaction } from "$lib";
    import { MerchantsStore, AccountsStore } from '$lib/controllers';
    import { AppState, ExplorerState, toggleSelected, FormState } from '$lib/config'
	import { initEditForm } from "$lib/actions";

    export let item: Transaction;
    export let ref: TableStore;

    let style: string;
    let icon: Thumbnail = getIcon(item, ref)

    interface Thumbnail {
        icon: string;
        icon_color: string;
    }
    function getIcon(record: Transaction, iconref: TableStore){
        let recID = 0;
        let bgcolor = '#00DB99';
        let imgsrc = ""; // /merchants/default.jpg";
        let store: Merchant[] | Bank[] = [];
        if (iconref === 'Merchants'){
           recID = record.merchant_id as number;
           store = $MerchantsStore.records;
        } else if (iconref === "Banks"){
            for(const acct of $AccountsStore.records){
                if (acct.id == record.account_id){
                    recID = acct.bank_id
                }
            }
            store = $AccountsStore.banks
        }
        
        for(const rec of store){
            if (rec.id == recID){
                if (rec.icon_color!=='Default'){ // && rec.icon !== 'Default'){
                    bgcolor = rec.icon_color;
                }
                if (rec.icon !== 'Default' && rec.icon !== ''){ // && rec.icon !== 'Default'){
                    imgsrc = rec.icon;
                }
            }
        }
        let bg = `background: ${bgcolor};`
        let thumbnail: Thumbnail = {icon: imgsrc, icon_color: bg}
        return thumbnail
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
    
    $: if ($AppState.theme === 'light'){
        style = `background: #bebdbf4d; text-shadow: 0px 0px 4px #ffffffb3; border-top: 1px solid #00000026; border-bottom: 1px solid #00000026; border-left: 1px solid #00000026;`
    } else {
        style = `background: #ffffff66; text-shadow: 0px 0px 4px #00000033; border-top: 1px solid #ffffff26; border-bottom: 1px solid #ffffff26; border-left: 1px solid #ffffff26;`
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
{#if $AppState.os === 'darwin'}
    {#if $FormState}
        <td class="MacCell" {style}>
            <div style="{icon.icon_color}" class="MacThumbnail" >
            {#if (icon.icon !== '')}
                <img src="{icon.icon}" alt=""/>
            {/if}
            </div>
        </td>
    {:else}
        <td class="MacCell" {style} on:click={()=>{editItem(item)}}>
            <div style="{icon.icon_color}" class="MacThumbnail" >
            {#if (icon.icon !== '')}
                <img src="{icon.icon}" alt=""/>
            {/if}
            </div>
        </td>
    {/if}
{:else if $AppState.os === 'win32'}
    {#if $FormState}
        <td class="WinCell" {style}>
            <div style="{icon.icon_color}" class="WinThumbnail">
            {#if (icon.icon !== '')}
                <img src="{icon.icon}" alt=""/>
            {/if}
            </div>
        </td>
    {:else}
        <td class="WinCell" {style} on:click={()=>{editItem(item)}}>
            <div style="{icon.icon_color}" class="WinThumbnail">
            {#if (icon.icon !== '')}
                <img src="{icon.icon}" alt=""/>
            {/if}
            </div>
        </td>
    {/if}
{/if}

<style>
    td {
        display: table-cell;
        box-sizing: border-box;
        border-radius: 8px 0px 0px 8px;
        padding: 5px 10px 5px 5px;
    }
    div {
        display: flex;
        box-sizing: border-box;
        cursor: pointer;
    }
    .MacCell {
        height: 35px;
    }
    .WinCell {
        height: 30px;
    }
    .MacThumbnail {
        height: 25px;
        width: 25px;
        border-radius: 6px;
    }
    .WinThumbnail {
        height: 20px;
        width: 20px;
        border-radius: 5px;
        background: #ffffff26;
    }
</style>