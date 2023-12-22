<script lang="ts">
	import { AppState } from '$lib/config';

    let style: string;
    
    $: if ($AppState.theme === 'light'){
        style = "background: #FFFFFF;"
    } else {
        style = "background: #4D4D4D;"
    }
</script>

<div {style} class="views">
    <slot></slot>
</div>

<style>
    .views {
        box-sizing: border-box;
        width: 100%;
        justify-content: center;
        overflow-y: hidden;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        align-self: stretch;
        border-radius: 8px;
        box-shadow: -2px -2px 3px 0px rgba(0, 0, 0, 0.10) inset, 1px 1px 3px 0px rgba(0, 0, 0, 0.10);
    }
</style>