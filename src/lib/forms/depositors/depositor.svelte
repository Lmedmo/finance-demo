<script lang='ts'>
	import { Buttons, Cancel, Delete, Field, Submit, Textbox, Trash, Fields } from "$lib/components/form";
	import { FormConfig, setCollection } from "$lib/config";
	import type { Depositor } from "$lib/models";

    let ft = $FormConfig.formtype
    let fc = $FormConfig.collection as Depositor;
    $: fc

    let depositor: Depositor
    let name = fc.name;

    $: name

    $: depositor = {
        id: fc.id,
        name: name
    }
</script>

<Fields>
    <Field name="Name">
        <Textbox bind:text={name} on:textSet={()=>setCollection(depositor)}/>
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
