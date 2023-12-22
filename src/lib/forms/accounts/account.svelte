<script lang="ts">
    import { FormConfig } from "$lib/config";
	import CreditCard from "./creditcard.svelte";
	import Checking from "./checking.svelte";
	import Savings from "./savings.svelte";
    import Mobile from "./mobile.svelte";
    import Investment from "./investment.svelte";

    $: rt = $FormConfig.rectype;
</script>

{#if rt === 'Credit Card'}
    <CreditCard />
{:else if rt === 'Checking Account'}
    <Checking />
{:else if rt === 'Savings Account'}
    <Savings />
{:else if rt === 'Mobile Account'}
    <Mobile />
{:else if rt === 'Investment Account'}
    <Investment />
{/if}