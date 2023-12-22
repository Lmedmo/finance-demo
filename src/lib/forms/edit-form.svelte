<script lang="ts">
	import { FormConfig } from "$lib/config";
	import { AccountForm, BudgetForm, CategoryForm, DepositorForm, MerchantForm, TransactionForm } from "$lib/forms";
    import { Title } from '$lib/components/form';

    $: rt = $FormConfig.rectype
</script>

<div class="Edit-Form" >
    <Title text={`Edit ${rt}`} />
    {#if rt === "Transaction" || rt === "Expense" || rt === 'Deposit' || rt === 'Transfer'}
        <TransactionForm />
    {:else if rt === "Account" || rt === 'Checking Account' || rt === 'Savings Account' || rt === 'Mobile Account' || rt === 'Credit Card' || rt === 'Investment Account'}
        <AccountForm />
    {:else if rt === "Budget"}
        <BudgetForm />
    {:else if rt === "Category"}
        <CategoryForm />
    {:else if rt === "Depositor"}
        <DepositorForm />
    {:else if rt === "Merchant"}
        <MerchantForm />
    {/if}
</div>

<style>
    .Edit-Form {
        background-color: rgba(156, 156, 156, 0.6);
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        border-radius: 8px;
        backdrop-filter: blur(20px);
        box-shadow: 3px 3px 10px 1px rgba(0, 0, 0, 0.249);
        border: 2px solid #d1d1d140;
        align-items: center;
        justify-content: flex-start;
        align-self: stretch;
        padding-bottom: 10px;
        flex: 1 0;
        overflow: hidden;
        overflow-y: hidden;
    }
</style>