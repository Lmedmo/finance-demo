<script lang="ts">
	import Planning from './planning/+page.svelte';
	import Activity from './activity/+page.svelte';
    import Settings from './settings/+page.svelte';
	import { PageState } from '$lib';
</script>
    
{#if $PageState.id === 1}
    <Planning />
{:else if $PageState.id === 2}
    <Activity />
{:else if $PageState.id === 0}
    <Settings />
{:else}
    <p>Error loading page</p>
{/if}