<script lang="ts">
    import Tab from './tab.svelte';
	import { fetchPages } from '$lib/config';

	$: pages = fetchPages();
</script>

<div data-tauri-drag-region class="Tabs">
    {#each pages as page}
		<Tab {page}/>
	{/each}
</div>

<style>
    .Tabs {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
        gap: 10px;
    }
</style>