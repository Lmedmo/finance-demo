<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
	import { AppState } from '$lib/config';

    let hovered: boolean;
    let style: string;
    let closefill: string;

    const closeApp = () => {
		appWindow.close();
	};

	const hover = () => { hovered = true }
    const unhover = () => { hovered = false }

    $: if (hovered) {
		style = `background: #FE1500; scale: 105%;`;
		closefill = `#7A0A00`;
	} else if ($AppState.theme === 'light'){
		style = `background: linear-gradient(0deg, rgba(254, 21, 0, 0.30) 0%, rgba(254, 21, 0, 0.30) 100%), rgba(255, 255, 255, 0.60);`;
		closefill = `#FE1500`;
	} else {
		style = `background: linear-gradient(0deg, rgba(254, 21, 0, 0.30) 0%, rgba(254, 21, 0, 0.30) 100%), rgba(0, 0, 0, 0.30);`;
		closefill = `#FE1500`;
	};
</script>

{#if $AppState.os === 'darwin'}
<button {style} class="Macbutton" on:click={closeApp} on:mouseenter={hover} on:mouseleave={unhover}>
    <svg xmlns="http://www.w3.org/2000/svg" width="17" height="17" viewBox="0 0 17 17" fill="none">
        <path d="M3.55025 1.42893C2.96447 0.843147 2.01472 0.843147 1.42893 1.42893C0.843147 2.01472 0.843147 2.96447 1.42893 3.55025L6.37868 8.5L1.42894 13.4497C0.843149 14.0355 0.843148 14.9853 1.42894 15.5711C2.01472 16.1569 2.96447 16.1569 3.55025 15.5711L8.5 10.6213L13.4497 15.5711C14.0355 16.1569 14.9853 16.1569 15.5711 15.5711C16.1569 14.9853 16.1569 14.0355 15.5711 13.4497L10.6213 8.5L15.5711 3.55025C16.1569 2.96447 16.1569 2.01472 15.5711 1.42893C14.9853 0.843147 14.0355 0.843147 13.4498 1.42893L8.5 6.37868L3.55025 1.42893Z" fill="{closefill}"/>
    </svg>
</button>
{:else if ($AppState.os === 'win32')}
<button {style} class="Winbutton" on:click={closeApp} on:mouseenter={hover} on:mouseleave={unhover}>
    <svg xmlns="http://www.w3.org/2000/svg" width="12" height="11" viewBox="0 0 12 12" fill="none">
        <path d="M2.0402 0.343141C1.57157 -0.125488 0.81177 -0.125489 0.34314 0.343141C-0.125489 0.81177 -0.125489 1.57157 0.34314 2.0402L4.30294 6L0.343142 9.95979C-0.125487 10.4284 -0.125487 11.1882 0.343142 11.6568C0.811771 12.1255 1.57157 12.1255 2.0402 11.6568L6 7.69705L9.95979 11.6568C10.4284 12.1255 11.1882 12.1255 11.6568 11.6568C12.1255 11.1882 12.1255 10.4284 11.6568 9.95979L7.69705 5.99999L11.6568 2.0402C12.1255 1.57157 12.1255 0.81177 11.6568 0.343141C11.1882 -0.125488 10.4284 -0.125488 9.95979 0.343141L6 4.30294L2.0402 0.343141Z" fill="{closefill}"/>
    </svg>
</button>
{:else}
<button {style} class="Winbutton" on:click={closeApp} on:mouseenter={hover} on:mouseleave={unhover}>
    X
</button>
{/if}

<style>
    .Winbutton {
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
		justify-content: center;
		align-items: center;
		flex-shrink: 0;
		border-radius: 10px;
		border: 0.25px solid #ffffff26;
		box-shadow: 2px 2px 5px 0px #00000026;
		cursor: pointer;
        height: 20px;
        width: 20px;
	}
    .Macbutton {
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
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
	svg {
		display: flex;
		align-items: center;
		justify-content: center;
		box-sizing: border-box;
	}
</style>
