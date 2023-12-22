<script lang="ts">
	import Tabs from './tabs.svelte';
	import Buttons from './buttons.svelte';
	import Title from './title.svelte';
	import { AppState } from '$lib/config';
</script>

{#if $AppState.os === 'darwin'}
	<div data-tauri-drag-region class="MacTitlebar">
		<Title />
		<Tabs />
		<Buttons />
	</div>
{:else if $AppState.os === 'win32'}
	<div data-tauri-drag-region class="WinTitlebar">
		<Title />
		<Tabs />
		<Buttons />
	</div>
{/if}

<style>
    .MacTitlebar {
		/* background-color: beige; */
		display: flex;
		flex-direction: row;
		box-sizing: border-box;
		border-radius: 12px 12px 0px 0px;
		justify-content: space-between;
		align-items: center;
		height: 30px;
	}
	.WinTitlebar {
		/* background-color: beige; */
		display: flex;
		flex-direction: row;
		box-sizing: border-box;
		border-radius: 12px 12px 0px 0px;
		justify-content: space-between;
		align-items: center;
		height: 25px;
	}
</style>