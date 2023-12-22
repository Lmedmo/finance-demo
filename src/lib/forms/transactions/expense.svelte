<script lang="ts">
    import type { Transaction, Explorer, SelectorOption, ExplOption } from '$lib'
    import { FormConfig, setCollection } from '$lib/config'
    import { AccountsStore, CategoriesStore, MerchantsStore, TransactionsStore } from '$lib/controllers'
    import { Buttons, Cancel, Currency, Datepicker, Delete, Field, Selector, Submit, Textbox, Trash, Fields } from "$lib/components/form";
    
    let expl = $FormConfig.explorer as Explorer;
    
    let ft =$FormConfig.formtype
    let fc = $FormConfig.collection as Transaction;
    $: fc

    let transaction: Transaction;
    let memo = {id: fc.memo_id, name: fc.memo_name} as SelectorOption; 
    let acct = {id: fc.account_id, name: fc.account_name};
    let date = fc.date as string;
    let merch = {id: fc.merchant_id, name: fc.merchant_name} as SelectorOption;
    let desc = fc.description as string;
    let cat = {id: fc.category_id, name: fc.category_name} as SelectorOption;

    let raw_amt = fc.amount as number;
    let rnd_amt = (Math.round(raw_amt * 100) / 100).toFixed(2)
    let split_amt = (rnd_amt+'').split('.');

    let dollars: number = Number(split_amt[0]);
    let cents: number = Number(split_amt[1]);
    let amt: number;

    $: memo
    $: acct
    $: merch
    $: desc
    $: cat
    $: date
    $: dollars
    $: cents
    $: amt = parseFloat(`${dollars}.${cents}`)

    if (expl.references === "Accounts" && expl.selected.id != 0){
        acct = expl.selected as ExplOption
    }
    
    $: transaction = {
        id: fc.id,
        type_id: $TransactionsStore.types[2].id, // Withdrawal (id=3)
        type_name: $TransactionsStore.types[2].name, // Withdrawal (id=3)
        memo_id: memo.id,
        memo_name: memo.name,
        account_id: acct.id as number,
        account_name: acct.name,
        to_from_account: undefined,
        to_from_acct_name: undefined,
        date: date+'',
        merchant_id: merch.id,
        merchant_name: merch.name,
        depositor_id: undefined,
        depositor_name: undefined,
        description: desc,
        category_id: cat.id,
        category_name: cat.name,
        amount: amt
    }
</script>

<Fields>
    <Field name="Amount">
        <Currency bind:cents={cents} bind:dollars={dollars} on:amtSet={()=>setCollection(transaction)}/>
    </Field>

    <Field name="Memo">
        <Selector store={$TransactionsStore.memos} bind:selected={memo} on:optSet={()=>setCollection(transaction)} />
    </Field>

    {#if (expl.references !== "Accounts" || (expl.references === "Accounts" && expl.selected.id == 0)) && $FormConfig.formtype === 'Entry'}
    <Field name="Account">
        <Selector store={$AccountsStore.records} bind:selected={acct} on:optSet={()=>setCollection(transaction)} />
    </Field>
    {/if}

    <Field name="Date">
        <Datepicker bind:date={date} on:dateSet={()=>setCollection(transaction)}/>
    </Field>

    <Field name="Merchant">
        <Selector store={$MerchantsStore.records} bind:selected={merch} on:optSet={()=>setCollection(transaction)} />
    </Field>

    <Field name="Description">
        <Textbox bind:text={desc} on:textSet={()=>setCollection(transaction)}/>
    </Field>

    <Field name="Category">
        <Selector store={$CategoriesStore.records} bind:selected={cat} on:optSet={()=>setCollection(transaction)} />
    </Field>
</Fields>

<Buttons>
    <Cancel />
    {#if ft === 'Entry'}
        <Submit />
    {:else if ft === 'Edit'}
        <Submit />
        <Trash />
    {:else if ft === 'Delete'}
        <Delete />
    {/if}
</Buttons>
