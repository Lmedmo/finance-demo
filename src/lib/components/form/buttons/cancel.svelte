<script lang="ts">
	import { AppState, closeForm } from "$lib/config";

    let style: string;
    let hovered: boolean;

    const hover = () => { hovered = true }
    const unhover = () => { hovered = false }

    $: hoverStyle = `background: white; color: ${$AppState.color}; font-family: ${$AppState.font};`;
    $: normalStyle = `background: ${$AppState.color}; color: ${$AppState.fill}; font-family: ${$AppState.font};`;

    $: if (hovered) {style = hoverStyle} else {style = normalStyle};
</script>

<button class="Normal" {style} on:mouseenter={hover} on:mouseleave={unhover} on:click={closeForm}>Cancel</button>

<style>
    .Normal {
        border-radius: 5px;
        border: none;
        padding: 10px 0px;
        flex: 1 0;
        text-decoration: none;
        cursor: pointer;
        font-size: 1.2rem;
        font-weight: 600;
    }
</style>