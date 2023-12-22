<script lang="ts">
    import { AppState } from "$lib";

    let style: string;
    $: if ($AppState.os === 'darwin'){
        style = `height: 35px;`
    } else {
        style = `height: 31.5px;`
    }
</script>

<div class="Container">
    <div class="Toolbar" {style}/>
    <div class="Buttons">
        <slot></slot>
    </div>
</div>

<style>
    .Container {
        /* background-color: aqua; */
        position: relative;
        display: flex;
        flex-direction: column;
        align-self: stretch;
        z-index: 0;
    }
    .Toolbar {
        position: relative;
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        border-radius: 8px;
        top: 0px;
        right: 0px;
        left:0px;
        border: 1px solid #4d4d4d40;
        background: #00000080;
        box-shadow: 0px 2px 4px 0px #00000036;
        backdrop-filter: blur(2px);
        z-index: 1;
    }
    .Buttons {
        position: absolute;
        top: 0px;
        left: 0px;
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        padding: 4px;
        align-items: flex-start;
        gap: 10px;
        z-index: 2;
    }
</style>