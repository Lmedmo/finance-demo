<script lang="ts">
    import type { Transaction, Explorer, SelectorOption, ExplOption } from '$lib'
    import { FormConfig, setCollection } from '$lib/config'
    import { AccountsStore, DepositorStore, TransactionsStore } from '$lib/controllers'
    import { Buttons, Cancel, Currency, Datepicker, Delete, Field, Selector, Submit, Textbox, Trash, Fields } from "$lib/components/form";

    let expl = $FormConfig.explorer as Explorer;

    let ft = $FormConfig.formtype
    let fc = $FormConfig.collection as Transaction;
    $: fc

    let transaction: Transaction;
    let memo = {id: fc.memo_id, name: fc.memo_name} as SelectorOption; 
    let acct = {id: fc.account_id, name: fc.account_name};
    let date = fc.date as string;
    let desc = fc.description as string;
    let depos = {id: fc.depositor_id, name: fc.depositor_name} as SelectorOption;

    let raw_amt = fc.amount as number;
    let rnd_amt = (Math.round(raw_amt * 100) / 100).toFixed(2)
    let split_amt = (rnd_amt+'').split('.');

    let dollars: number = Number(split_amt[0]);
    let cents: number = Number(split_amt[1]);
    let amt: number;

    $: memo
    $: acct
    $: desc
    $: date
    $: depos    
    $: dollars
    $: cents
    $: amt = parseFloat(`${dollars}.${cents}`)
    
    if (expl.references === "Accounts" && expl.selected.id != 0){
        acct = expl.selected as ExplOption
    }

    $: transaction = {
        id: fc.id,
        type_id: $TransactionsStore.types[1].id, // Deposit (id=2)
        type_name: $TransactionsStore.types[1].name, // Deposit (name=Deposit)
        memo_id: memo.id,
        memo_name: memo.name,
        account_id: acct.id as number,
        account_name: acct.name,
        to_from_account: undefined,
        to_from_acct_name: undefined,
        date: date,
        merchant_id: undefined,
        merchant_name: undefined,
        depositor_id: depos.id,
        depositor_name: depos.name,
        description: desc,
        category_id: undefined,
        category_name: undefined,
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

    {#if expl.references !== "Accounts" || (expl.references === "Accounts" && expl.selected.id == 0)}
    <Field name="Account">
        <Selector store={$AccountsStore.records} bind:selected={acct} on:optSet={()=>setCollection(transaction)} />
    </Field>
    {/if}

    <Field name="Date">
        <Datepicker bind:date={date} on:dateSet={()=>setCollection(transaction)}/>
    </Field>

    <Field name="Depositor">
        <Selector store={$DepositorStore.records} bind:selected={depos} on:optSet={()=>setCollection(transaction)} />
    </Field>


    <Field name="Description">
        <Textbox bind:text={desc} on:textSet={()=>setCollection(transaction)}/>
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
