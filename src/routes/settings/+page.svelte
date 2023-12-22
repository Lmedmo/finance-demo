<script lang="ts">
    import Profile from './profile/+page.svelte';
    import Preferences from './preferences/+page.svelte';
    import { Views } from '$lib/layouts';
	import { ViewState } from '$lib';
</script>

<Views>
    {#if $ViewState.id === 1}
        <Preferences />
    {:else if $ViewState.id === 2}
        <Profile />
    {:else}
        <div>Invalid Settings state</div>
    {/if}
</Views>