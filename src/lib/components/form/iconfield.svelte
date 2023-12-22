<script lang="ts">
	import { AppState } from "$lib/config";
	import type { BudgetCategory } from "$lib/models";

    export let item: BudgetCategory;

    let style: string;
    let iconStyle = `background: ${item.icon_color};`;
    $: if ($AppState.os === 'darwin'){
            style = `font-family: ${$AppState.font}; font-size: 1.1rem; font-weight: 400; color: white;`; //${$AppState.text};`;
        } else if ($AppState.os === 'win32'){
            style = `font-family: ${$AppState.font}; font-size: .9rem; font-weight: 500; color: white;`; //${$AppState.text};`;
        }
</script>

<div class="Field">
    <div class="Lable">
        <img class="Icon" src="{item.icon}" alt="account icon" style={iconStyle}>
        <div class="Lable" {style}>{item.name}</div>
    </div>
    <slot></slot>
</div>

<style>
    .Field {
        background-color: #5a5a5a;
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        justify-content: space-between;
        align-items: center;
        border-radius: 5px;
        padding: 10px;
    }
    .Lable {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        align-items: center;
        justify-content: center;
        gap: 15px;
    }
    .Icon {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        align-items: center;
        justify-content: center;
        max-width: 30px;
        max-height: 30px;
        border-radius: 7px;
    }
</style>