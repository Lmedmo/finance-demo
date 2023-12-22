<script lang="ts">
	import { Colors } from "$lib";
	import { AppState } from "$lib";
	import { deleteRecs } from "$lib/actions";

    let style: string;
    let hovered: boolean;

    const hover = () => { hovered = true }
    const unhover = () => { hovered = false }
    const submit = () => {
        deleteRecs()
    }

    $: hoverStyle = `background: white; color: ${Colors.scarlet}; font-family: ${$AppState.font};`;
    $: normalStyle = `background: ${Colors.scarlet}; color: black; font-family: ${$AppState.font};`;

    $: if (hovered) {style = hoverStyle} else {style = normalStyle};
</script>

<button class="Normal" {style} on:mouseenter={hover} on:mouseleave={unhover} on:click={submit}>Delete</button>

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