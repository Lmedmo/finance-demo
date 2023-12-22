<script lang="ts">
    import { AppState } from "$lib/config";

    export let text: String;

    let style: string;

    $: if ($AppState.os === 'darwin'){
        style = `font-family: ${$AppState.font}; color: ${$AppState.text}; font-size: 2.5rem;`
    } else if ($AppState.os === 'win32') {
        style = `font-family: ${$AppState.font}; color: ${$AppState.text}; font-size: 2.1rem;`
    }
</script>

<div class="Title" {style}>{text}</div>

<style>
    .Title {
        position: sticky;
        top: 0px;
        display: flex;
        padding: 5%;
        align-self: stretch;
        justify-content: center;
        box-sizing: border-box;
        font-weight: bold;
    }
</style>