<script lang="ts">
	import Settings from './settings.svelte';
	import Close from './close.svelte';
	import Max from './max.svelte';
	import Min from './min.svelte';
</script>

<div class="Buttons" data-tauri-drag-region>
	<Settings />
	<div class="Divider"/>
	<Close />
	<Min />
	<Max />
</div>

<style>
    .Buttons {
		/* background-color: aqua; */
		display: flex;
		flex-direction: row;
		box-sizing: border-box;
		align-items: flex-start;
		justify-content: center;
		align-self: stretch;
		gap: 9px;
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