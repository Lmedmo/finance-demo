<script lang="ts">
    import type { Account } from '$lib/models/account'
    import { AccountsStore, FormConfig, CurrentUser, setCollection, getBank } from "$lib";
    import { Field, Currency, Selector, Textbox, Buttons, Cancel, Submit, Trash, Fields, Delete } from "$lib/components/form";

    let ft = $FormConfig.formtype
    let fc = $FormConfig.collection as Account;
    $: fc

    let account: Account;
    let name = fc.name;
    let bank = getBank(fc.bank_id);
    let acctUsrname = fc.account_username;

    let raw_amt = fc.initial_balance as number;
    let rnd_amt = (Math.round(raw_amt * 100) / 100).toFixed(2)
    let split_amt = (rnd_amt+'').split('.');

    let dollars: number = Number(split_amt[0]);
    let cents: number = Number(split_amt[1]);
    let init_bal: number;

    $: name
    $: bank
    $: acctUsrname
    $: dollars
    $: cents
    $: init_bal = parseFloat(`${dollars}.${cents}`)

    $: account = {
        id: fc.id,
        name: name,
        bank_id: bank.id,
        user_id: $CurrentUser.id,
        type_id: $AccountsStore.types[1].id, // Mobile Account (id=2),
        initial_balance: init_bal,
        credit_limit: undefined,
        due_date: undefined,
        interest_rate: undefined,
        cfreq: undefined,
        period_unit: undefined,
        account_username: acctUsrname,
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
        <Currency bind:cents={cents} bind:dollars={dollars} on:amtSet={()=>setCollection(account)}/>
    </Field>

    <Field name="App Username">
        <Textbox bind:text={acctUsrname} on:textSet={()=>setCollection(account)}/>
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