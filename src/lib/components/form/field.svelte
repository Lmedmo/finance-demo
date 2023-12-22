<script lang="ts">
	import { AppState } from "$lib/config";

    export let name: string;

    let style: string;
    $: if ($AppState.os === 'darwin'){
            style = `font-family: ${$AppState.font}; font-size: 1.1rem; font-weight: 400; color: white;`;//${$AppState.text};`;
        } else if ($AppState.os === 'win32'){
            style = `font-family: ${$AppState.font}; font-size: .9rem; font-weight: 500; color: white;`;//${$AppState.text};`;
        }
</script>

<div class="Field">
    <div class="Lable" {style}>{name}</div>
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
</style>