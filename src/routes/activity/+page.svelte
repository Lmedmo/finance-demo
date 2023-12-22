<script lang="ts">
    import { ViewState } from '$lib/config'
    import { Views } from '$lib/layouts';
    import Balances from './balances/+page.svelte';
</script>

<Views>
    {#if $ViewState.id === 1}
        <Balances />
    {:else}
        <p>Error loading view</p>
    {/if}
</Views>