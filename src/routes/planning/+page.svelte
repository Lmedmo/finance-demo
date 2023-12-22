<script lang="ts">
    import { ViewState } from '$lib/config'
    import { Views } from '$lib/layouts';
    import Budgets from './budgets/+page.svelte';
</script>

<Views>
    {#if $ViewState.id === 1}
        <Budgets />
    {:else}
        <p>Error loading view</p>
    {/if}
</Views>
    