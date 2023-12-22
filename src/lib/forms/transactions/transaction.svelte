<script lang="ts">
    import { FormConfig } from "$lib/config";
	import Expense from "./expense.svelte";
	import Deposit from "./deposit.svelte";
	import Transfer from "./transfer.svelte";

    $: rt = $FormConfig.rectype;
</script>

{#if rt === 'Expense'}
    <Expense />
{:else if rt === 'Deposit'}
    <Deposit />
{:else if rt === 'Transfer'}
    <Transfer />
{/if}
