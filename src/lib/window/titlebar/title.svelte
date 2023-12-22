<script lang="ts">
	import { AppState } from '$lib/config';

	$: textstyle = `font-family: ${$AppState.font}; color: ${$AppState.text};`;
</script>

<div class="Title"  data-tauri-drag-region>
	{#if $AppState.os === 'darwin'}
		<img class="MacIcon" src="/icons/icon.png" alt="finance_icon" data-tauri-drag-region/>
		<div class="MacAppName" style="{textstyle}" data-tauri-drag-region>Finance</div>
	{:else if $AppState.os === 'win32'}
		<img class="WinIcon" src="/icons/icon.png" alt="finance_icon" data-tauri-drag-region/>
		<div class="WinAppName" style="{textstyle}" data-tauri-drag-region>Finance</div>
	{/if}
	<div class="Divider"/>
</div>

<style>
    .Title {
		/* background-color: rgb(96, 174, 13); */
		display: flex;
		flex-direction: row;
		box-sizing: border-box;
		align-items: flex-start;
		justify-content: center;
		align-self: stretch;
		gap: 9px;
	}
/* Mac */
    .MacIcon {
		border-radius: 6px;
		height: 25px;
		width: 25px;
	}
	.MacAppName {
		font-size: 1.2rem; 
		font-weight: 400;
		text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.15);
	}
/* Windows */
	.WinIcon {
		border-radius: 6px;
		height: 20px;
		width: 20px;
	}
	.WinAppName {
		font-size: .9rem;
		font-weight: 600;
		text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.15);
	}
	.Divider {
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        background: #BEBDBF;
        width: 2px;
        justify-content: center;
        align-items: center;
        align-self: stretch;
		margin: 0px 0px 5px 0px;
		border-radius: 1px;
    }
</style>