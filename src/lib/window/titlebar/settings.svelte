<script lang="ts">
	import { CurrentUser } from "$lib";
	import { AppState, FormState, PageState } from "$lib/config";

    const toggleSettings = () => {
		let updated = {id: 0, title: "Settings"}
		PageState.set(updated);
	}
</script>

{#if $AppState.os === 'darwin'}
	{#if $FormState}
	<button style="cursor: default;" class="MacSettings">
		<img class="MacSettings" src="{$CurrentUser.icon}" alt="">
	</button>
	{:else}
	<button on:click={toggleSettings} class="MacSettings">
		<img class="MacSettings" src="{$CurrentUser.icon}" alt="">
	</button>
	{/if}
{:else if $AppState.os === 'win32'}
	{#if $FormState}
	<button style="cursor: default;" class="WinSettings">
		<img class="WinSettings" src="{$CurrentUser.icon}" alt="">
	</button>
	{:else}
	<button on:click={toggleSettings} class="WinSettings">
		<img class="WinSettings" src="{$CurrentUser.icon}" alt="">
	</button>
	{/if}
{/if}

<style>
    .MacSettings {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex-shrink: 0;
		border-radius: 12.5px;
		/* border: 0.25px solid #ffffff26; */
		border: none;
		box-shadow: 2px 2px 5px 0px #00000026;
		cursor: pointer;
        height: 25px;
        width: 25px;
	}
    .WinSettings {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex-shrink: 0;
		border-radius: 12.5px;
		/* border: 0.25px solid #ffffff26; */
		border: none;
		box-shadow: 2px 2px 5px 0px #00000026;
		cursor: pointer;
        height: 20px;
        width: 20px;
	}
</style>