<script lang="ts">
    import type { Account } from '$lib/models/account'
    import { AccountsStore, FormConfig, CurrentUser, setCollection, getBank, getCompFreq } from "$lib";
    import { Field, Currency, Selector, Textbox, Percentage, RadioButtons, Buttons, Cancel, Submit, Trash, Fields, Delete } from "$lib/components/form";

    let ft = $FormConfig.formtype
    let fc = $FormConfig.collection as Account;
    $: fc
    
    let account: Account;
    let name = fc.name;
    let bank = getBank(fc.bank_id);
    let rate = fc.interest_rate;
    let cfreq = getCompFreq(fc.compound_frequency as number);

    let raw_amt = fc.initial_balance as number;
    let rnd_amt = (Math.round(raw_amt * 100) / 100).toFixed(2)
    let split_amt = (rnd_amt+'').split('.');

    let dollars: number = Number(split_amt[0]);
    let cents: number = Number(split_amt[1]);
    let init_bal: number;
    
    $: name
    $: bank
    $: rate
    $: cfreq
    $: dollars
    $: cents
    $: acctnum = fc.account_number;
    $: rtnum = fc.routing_number;
    $: init_bal = parseFloat(`${dollars}.${cents}`)

    $: account = {
        id: fc.id,
        name: name,
        bank_id: bank.id,
        user_id: $CurrentUser.id,
        type_id: $AccountsStore.types[3].id, // Savings Account (id=4)
        initial_balance: init_bal,
        credit_limit: undefined,
        due_date: undefined,
        interest_rate: rate,
        compound_frequency: cfreq.id,
        period_unit: fc.period_unit,
        account_username: undefined,
        account_number: acctnum,
        routing_number: rtnum,
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
        <Currency bind:cents={cents} bind:dollars={dollars} on:amtSet={()=>setCollection(account)}/>
    </Field>

    <Field name="Interest Rate">
        <Percentage bind:rate={rate} on:rateSet={()=>setCollection(account)}/>
    </Field>

    <Field name="Compounding Frequency">
        <Selector store={$AccountsStore.cfreqs} bind:selected={cfreq} on:optSet={()=>setCollection(account)} />
    </Field>

    <Field name="Period Duration">
        <RadioButtons store={$AccountsStore.punits} fcindex={"period_unit"} srindex={"name"} />
    </Field>

    <Field name="Account Number">
        <Textbox bind:text={acctnum} on:textSet={()=>setCollection(account)}/>
    </Field>

    <Field name="Routing Number">
        <Textbox bind:text={rtnum} on:textSet={()=>setCollection(account)}/>
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