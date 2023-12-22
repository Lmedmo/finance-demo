<script lang="ts">
	import { addRec, editRec } from "$lib/actions";
	import { AppState, FormConfig } from "$lib/config";

    let style: string;
    let hovered: boolean;

    const hover = () => { hovered = true }
    const unhover = () => { hovered = false }
    const submit = () => {
        if ($FormConfig.formtype === 'Edit'){
            editRec()
        } else if ($FormConfig.formtype === 'Entry'){
            addRec()
        }
    }

    $: hoverStyle = `background: white; color: ${$AppState.color}; font-family: ${$AppState.font};`;
    $: normalStyle = `background: ${$AppState.color}; color: ${$AppState.fill}; font-family: ${$AppState.font};`;

    $: if (hovered) {style = hoverStyle} else {style = normalStyle};
</script>

<button class="Normal" {style} on:mouseenter={hover} on:mouseleave={unhover} on:click={submit}>Submit</button>

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