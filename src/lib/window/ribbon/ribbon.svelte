<script lang="ts">
    import { AppState, PageState, fetchViews } from '$lib/config';
    import Button from './button.svelte'

    $: views = fetchViews($PageState);
    let style = `background-color: ${$AppState.color};`
</script>

<div {style} class="Ribbon">
    {#each views as view}
        <Button {view}/>
    {/each}
</div>

<style>
    .Ribbon {
        display: flex;
        flex-direction: row;
        width: 100%;
        box-sizing: border-box;
        border-radius: 8px;
        box-shadow: -2px -2px 2px 0px rgba(0, 0, 0, 0.15) inset;
        padding: 5px;
        align-items: center;
        gap: 24px;
        top: 40px;
        justify-content: flex-start;
    }
</style>