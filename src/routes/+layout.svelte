<script lang="ts">
	import { Titlebar, Ribbon, ExplorerMenu } from '$lib/window';
	import { DeleteForm, EditForm, EntryForm } from '$lib/forms';
	import { appWindow } from '@tauri-apps/api/window';
	import { fly } from 'svelte/transition';
	import { AppState, FormConfig, FormState, changeTheme, defaultFormConfig, getAppState, getTables, getWindowSize, type AppStateObj } from '$lib/config';
	import Cancel from '$lib/components/form/buttons/cancel.svelte';

	async function appInit() {
		await getWindowSize();
		await getTables();
		await getAppState();
		const unlisten = await appWindow.onThemeChanged(({ payload: theme }) => {
			if ($AppState.themeval === 'Default'){
				changeTheme("Default", $AppState)
			}
		});
		FormConfig.set(defaultFormConfig);
		FormState.set(false)
		return true
	}
	function getStyle(state: AppStateObj){
		let style = `border: 2px solid ${state.color}40; ${state.window_clr}`;
	 	return style
	}
	function getFormStyle(state: AppStateObj){
		let style = `font-family: ${state.font}; color: ${state.text};`;
		return style
	}
</script>

{#await appInit() then done}
{#if done}
<div class="Window" style={getStyle($AppState)}>
	<Titlebar />
	<Ribbon />
	<div class="Base">
	{#if $FormState}
		<div style="filter: blur(6px);" class="Pages">
			<ExplorerMenu />
			<slot></slot>
		</div>

		<div data-tauri-drag-region class="Form" in:fly={{ duration: 1000, y: 200 }}> 
		{#if ($FormConfig.formtype === "Entry")}
			<EntryForm />
		{:else if ($FormConfig.formtype === "Edit")}
			<EditForm />
		{:else if ($FormConfig.formtype === "Delete")}
			<DeleteForm />
		{:else if ($FormConfig.formtype === "Import")}
			<div class="Message" style={getFormStyle($AppState)}>Importing...</div>
			<!-- Loading animation here -->
		{:else if ($FormConfig.formtype === "")}
			<div class="Message" style={getFormStyle($AppState)}>No Formtype</div>
			<Cancel />
		{:else}
			<div class="Message" style={getFormStyle($AppState)}>Unexpected Formtype</div>
			<Cancel />
		{/if}
		</div>
	{:else}
		<div style="filter: blur(0px);" class="Pages">
			<ExplorerMenu />
			<slot></slot>
		</div>
	{/if}
	</div>
</div>
{/if}
{/await}

<style>
	.Window {
		position: absolute;
		z-index: 0;
		top: 0px;
		right: 0px;
		bottom: 1px;
		left: 0px;
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
		border-radius: 12px;
		padding: 5px;
		margin-right: 2px;
		align-items: stretch;
		justify-content: stretch;
		overflow-y: hidden;
        overflow-x: hidden;
	}
	.Pages {
		position: relative;
		z-index: 0;
		display: flex;
		flex-direction: row;
		box-sizing: border-box;
		border-radius: 8px;
		margin-top: 2.5px;
		padding-top: 2.5px;
		flex: 1 0 ;
		overflow-y: hidden;
		overflow-x: hidden;
	}
	.Base {
		display: flex;
		flex-direction: row;
		box-sizing: border-box;
		border-radius: 8px;
		flex: 1 0 ;
		align-items: stretch;
		justify-content: center;
		overflow-y: hidden;
		overflow-x: hidden;
	}
	.Form {
        /* background-color: #7c7c7ce9; */
        position: absolute;
        width: 70%;
        top: 10%;
        bottom: 10%;
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        align-items: center;
        justify-content: center;
        border-radius: 8px;
        overflow-y: hidden;
		overflow-x: hidden;
    }
    .Message {
        font-size: 1.3rem;
        font-weight: 500;
    }
</style>