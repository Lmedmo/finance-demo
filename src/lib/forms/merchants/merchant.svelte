<script lang='ts'>
	import { Buttons, Cancel, Delete, Field, Submit, Textbox, Trash, Fields } from "$lib/components/form";
	import { FormConfig, setCollection } from "$lib/config";
	import type { Merchant } from "$lib/models";

    let ft = $FormConfig.formtype
    let fc = $FormConfig.collection as Merchant;
    $: fc

    let merchant: Merchant
    let name = fc.name;

    $: name

    $: merchant = {
        id: fc.id,
        name: name,
        icon: fc.icon,
        icon_color: fc.icon_color
    }
</script>

<Fields>
    <Field name="Name">
        <Textbox bind:text={name} on:textSet={()=>setCollection(merchant)}/>
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
