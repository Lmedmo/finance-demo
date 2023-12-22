<script lang="ts">
	import { type Category, FormConfig, setCollection } from "$lib";
    import {Field, Textbox, Buttons, Cancel, Submit, Trash, Fields, Delete } from "$lib/components/form";

    let ft = $FormConfig.formtype
    let fc = $FormConfig.collection as Category;
    $: fc

    let category: Category;
    let name = fc.name;

    $: name

    $: category = {
        id: fc.id,
        name: name,
        icon: fc.icon,
        icon_color: fc.icon_color,
        type_id: 1
    }
</script>

<Fields>
    <Field name="Name">
        <Textbox bind:text={name} on:textSet={()=>setCollection(category)}/>
    </Field>
</Fields>

<Buttons>
    <Cancel />
    {#if ft === 'Entry'}
        <Submit />
    {:else if ft === 'Edit'}
        <Submit />
        {#if fc.type_id == 1}
            <Trash />
        {/if}
    {:else if ft === 'Delete'}
        <Delete />
    {/if}
</Buttons>
