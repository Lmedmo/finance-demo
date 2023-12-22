<script lang="ts">
	import type { TableColumn } from "$lib/config";
	import { AppState } from "$lib/config";
    
    export let columns: TableColumn[];

    $: style = `background-color: ${$AppState.color}; font-family: ${$AppState.font};`;
</script>

<tr>
    <th/>     
    {#if $AppState.os ==='darwin'}
        <th class="MacHeadingSpacer" {style}></th>
        {#each columns as col}
            <!-- <th class="MacHeading" {style}>{getHeading(data, col)}</th>  -->
            <th class="MacHeading" {style}>{col.headingname}</th> 
        {/each}
        <th class="MacHeadingEnd" {style}></th>
    {:else if $AppState.os === 'win32'}
        <th class="WinHeadingSpacer" {style}></th>
        {#each columns as col}
            <!-- <th class="WinHeading" {style}>{getHeading(data, col)}</th> -->
            <th class="WinHeading" {style}>{col.headingname}</th>  
        {/each}
        <th class="WinHeadingEnd" {style}></th>
    {/if}
</tr>

<style>
/* Mac */
    .MacHeading {
        display: table-cell;
        height: 35px;
        vertical-align: middle;
        color: white;
        font-size: 1.1rem;
        font-weight: 600;
        text-align: left;
    }
    .MacHeadingSpacer {
        height: 35px;
        border-radius: 8px 0px 0px 3px;
    }
    .MacHeadingEnd {
        height: 35px;
        border-radius: 0px 8px 3px 0px;
    }
/* Windows */
    .WinHeading {
        display: table-cell;
        height: 30px;
        vertical-align: middle;
        color: white;
        font-size: .9rem;
        font-weight: 600;
        text-align: left;
    }
    .WinHeadingSpacer {
        height: 30px;
        border-radius: 8px 0px 0px 3px;
    }
    .WinHeadingEnd {
        height: 30px;
        border-radius: 0px 8px 3px 0px;
    }
</style>