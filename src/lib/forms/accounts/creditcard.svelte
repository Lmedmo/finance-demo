<script lang="ts">
    import type { Account } from '$lib'
    import { AccountsStore, FormConfig, CurrentUser, setCollection, getBank } from "$lib";
    import { Field, Currency, Selector, Percentage, Datepicker, Textbox, Buttons, Cancel, Submit, Trash, Delete, Fields } from "$lib/components/form";

    let ft = $FormConfig.formtype
    let fc = $FormConfig.collection as Account;
    $: fc

    let account: Account;
    let name = fc.name;
    let bank = getBank(fc.bank_id);
    let rate = fc.interest_rate;
    let date = fc.due_date as string;

    let raw_ib = fc.initial_balance as number;
    let rnd_ib = (Math.round(raw_ib * 100) / 100).toFixed(2)
    let split_ib = (rnd_ib+'').split('.');

    let raw_cl = fc.credit_limit as number;
    let rnd_cl = (Math.round(raw_cl * 100) / 100).toFixed(2)
    let split_cl = (rnd_cl+'').split('.');

    let ib_dollars: number = Number(split_ib[0]);
    let ib_cents: number = Number(split_ib[1]);
    let cl_dollars: number = Number(split_cl[0]);
    let cl_cents: number = Number(split_cl[1]);
    let init_bal: number;
    let crlim: number;

    $: name
    $: bank
    $: rate
    $: date
    $: ib_dollars
    $: ib_cents
    $: cl_dollars
    $: cl_cents

    $: init_bal = parseFloat(`${ib_dollars}.${ib_cents}`)
    $: crlim = parseFloat(`${cl_dollars}.${cl_cents}`)

    $: account = {
        id: fc.id,
        name: name,
        bank_id: bank.id,
        user_id: $CurrentUser.id,
        type_id: $AccountsStore.types[0].id, // Credit Card (id=1)
        initial_balance: init_bal,
        credit_limit: crlim,
        due_date: date,
        interest_rate: rate,
        cfreq: undefined,
        period_unit: undefined,
        account_username: undefined,
        account_number: undefined,
        routing_number: undefined,
    }

</script>

<Fields>
    <Field name="Name">
        <Textbox bind:text={name} on:textSet={()=>setCollection(account)}/>
    </Field>

    <Field name="Bank">
        <Selector store={$AccountsStore.banks} bind:selected={bank} on:optSet={()=>setCollection(account)} />
    </Field>

    <Field name="Initial Balance">
        <Currency bind:cents={ib_cents} bind:dollars={ib_dollars} on:amtSet={()=>setCollection(account)}/>
    </Field>

    <Field name="Credit Limit">
        <Currency bind:cents={cl_cents} bind:dollars={cl_dollars} on:amtSet={()=>setCollection(account)}/>
    </Field>

    <Field name="Due Date">
        <Datepicker bind:date={date} on:dateSet={()=>setCollection(account)}/>
    </Field>

    <Field name="Interest Rate">
        <Percentage bind:rate={rate} on:rateSet={()=>setCollection(account)}/>
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
