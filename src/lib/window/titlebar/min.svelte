<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
	import { AppState } from '$lib/config';

	let hovered: boolean;
    let style: string;
	let minfill: string;

    const hover = () => { hovered = true }
    const unhover = () => { hovered = false }

	const minimizeApp = () => {
		appWindow.minimize();
	};

    $: if (hovered) {
		style = `background: #FFD600; scale: 105%;`;
		minfill = `#9F8600`;
	} else if ($AppState.theme === 'light'){
		style = `background: linear-gradient(0deg, rgba(255, 214, 0, 0.30) 0%, rgba(255, 245, 0, 0.30) 100%), rgba(255, 255, 255, 0.60);`;
		minfill = `#FFD600`;
	} else {
		style = `background: linear-gradient(0deg, rgba(255, 214, 0, 0.30) 0%, rgba(255, 245, 0, 0.30) 100%), rgba(0, 0, 0, 0.30);`;
		minfill = `#FFD600`;
	};
</script>

{#if $AppState.os === 'darwin'}
<button {style} class="Macbutton" on:click|stopPropagation={minimizeApp} on:mouseenter={hover} on:mouseleave={unhover}>
    <svg xmlns="http://www.w3.org/2000/svg" width="21" height="3" viewBox="0 0 21 3" fill="none">
        <path d="M0.5 1.5C0.5 0.671573 1.17157 0 2 0H19C19.8284 0 20.5 0.671573 20.5 1.5C20.5 2.32843 19.8284 3 19 3H2C1.17157 3 0.5 2.32843 0.5 1.5Z" fill="{minfill}"/>
    </svg>
</button>
{:else if ($AppState.os === 'win32')}
<button {style} class="Winbutton" on:click|stopPropagation={minimizeApp} on:mouseenter={hover} on:mouseleave={unhover}>
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="4" viewBox="0 0 16 4" fill="none">
        <path d="M0 2.00002C0 1.33728 0.537258 0.800018 1.2 0.800018H14.8C15.4627 0.800018 16 1.33728 16 2.00002C16 2.66276 15.4627 3.20002 14.8 3.20002H1.2C0.537258 3.20002 0 2.66276 0 2.00002Z" fill="{minfill}"/>
    </svg>
</button>
{:else}
<button {style} class="Winbutton" on:click|stopPropagation={minimizeApp} on:mouseenter={hover} on:mouseleave={unhover}>
    -
</button>
{/if}

<style>
    .Winbutton {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex-shrink: 0;
		border-radius: 12.5px;
		border: 0.25px solid #ffffff26;
		box-shadow: 2px 2px 5px 0px #00000026;
		cursor: pointer;
        height: 20px;
        width: 20px;
	}
    .Macbutton {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex-shrink: 0;
		border-radius: 12.5px;
		border: 0.25px solid #ffffff26;
		box-shadow: 2px 2px 5px 0px #00000026;
		cursor: pointer;
        height: 25px;
        width: 25px;
	}
</style>