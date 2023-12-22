<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
	import { AppState } from '$lib/config';

    let hovered: boolean;
    let style: string;
    let maxfill: string;

	const hover = () => { hovered = true }
    const unhover = () => { hovered = false }

    const maximizeApp = () => {
		appWindow.toggleMaximize();
	};

    $: if (hovered) {
		style = `background: #1CD545; scale: 105%;`;
		maxfill = `#00891E`;
	} else if ($AppState.theme === 'light'){
		style = `background: linear-gradient(0deg, rgba(28, 213, 69, 0.30) 0%, rgba(28, 213, 69, 0.30) 100%), rgba(255, 255, 255, 0.60);`;
		maxfill = `#1CD545`;
	} else {
		style = `background: linear-gradient(0deg, rgba(28, 213, 69, 0.30) 0%, rgba(28, 213, 69, 0.30) 100%), rgba(0, 0, 0, 0.30);`;
		maxfill = `#1CD545`;
	};
</script>

{#if $AppState.os === 'darwin'}
<button {style} class="Macbutton" on:click={maximizeApp} on:mouseenter={hover} on:mouseleave={unhover}>
    <svg xmlns="http://www.w3.org/2000/svg" width="17" height="17" viewBox="0 0 17 17" fill="none">
        <path d="M6.1118 13.0095L9.55167 13.0054C10.3843 13.0043 11.0585 13.6785 11.0575 14.5112C11.0565 15.3438 10.3807 16.0197 9.54801 16.0207L2.5123 16.0292C2.09146 16.0297 1.71109 15.8577 1.43793 15.58C1.43492 15.577 1.43192 15.5741 1.42893 15.5711C1.42595 15.5681 1.42298 15.5651 1.42003 15.5621C1.14228 15.2889 0.9703 14.9086 0.970811 14.4877L0.979353 7.45201C0.980364 6.61935 1.65618 5.94354 2.48884 5.94253C3.32149 5.94151 3.99567 6.61569 3.99466 7.44835L3.99048 10.8882L10.8882 3.99049L7.44833 3.99467C6.61568 3.99568 5.9415 3.3215 5.94251 2.48885C5.94352 1.6562 6.61934 0.980379 7.45199 0.979369L14.4877 0.970826C14.9086 0.970315 15.2889 1.1423 15.5621 1.42006C15.5651 1.423 15.5681 1.42597 15.5711 1.42895C15.574 1.43192 15.577 1.43491 15.5799 1.4379C15.8577 1.71107 16.0297 2.09145 16.0292 2.51232L16.0206 9.54803C16.0196 10.3807 15.3438 11.0565 14.5112 11.0575C13.6785 11.0585 13.0043 10.3843 13.0053 9.55168L13.0095 6.11181L6.1118 13.0095Z" fill="{maxfill}"/>
    </svg>
</button>
{:else if ($AppState.os === 'win32')}
<button {style} class="Winbutton" on:click={maximizeApp} on:mouseenter={hover} on:mouseleave={unhover}>
    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 14 14" fill="none">
        <path d="M5.08945 10.6076L7.84134 10.6043C8.50746 10.6035 9.04681 11.1428 9.046 11.8089C9.04519 12.4751 8.50454 13.0157 7.83841 13.0165L2.20985 13.0234C1.87317 13.0238 1.56888 12.8862 1.35035 12.664C1.34794 12.6616 1.34554 12.6593 1.34315 12.6569C1.34077 12.6545 1.33839 12.6521 1.33603 12.6497C1.11383 12.4311 0.976246 12.1268 0.976655 11.7902L0.983489 6.1616C0.984297 5.49548 1.52495 4.95482 2.19107 4.95401C2.8572 4.95321 3.39654 5.49255 3.39573 6.15867L3.39239 8.91057L8.91057 3.39239L6.15867 3.39573C5.49255 3.39654 4.95321 2.8572 4.95402 2.19107C4.95482 1.52495 5.49548 0.984297 6.1616 0.983489L11.7902 0.976655C12.1269 0.976245 12.4312 1.11383 12.6497 1.33604C12.6521 1.3384 12.6545 1.34077 12.6569 1.34315C12.6592 1.34553 12.6616 1.34792 12.664 1.35031C12.8862 1.56885 13.0238 1.87315 13.0234 2.20985L13.0165 7.83841C13.0157 8.50453 12.4751 9.04519 11.8089 9.046C11.1428 9.04681 10.6035 8.50746 10.6043 7.84134L10.6076 5.08945L5.08945 10.6076Z" fill="{maxfill}"/>
    </svg>
</button>
{:else}
<button {style} class="Winbutton" on:click={maximizeApp} on:mouseenter={hover} on:mouseleave={unhover}>
    +
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