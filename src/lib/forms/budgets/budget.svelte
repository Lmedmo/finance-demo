<script lang='ts'>
	import { CategoriesStore, getCatsAsBcats } from "$lib";
    import { IconField, Field, Checkbox, Fields, Buttons, Cancel, Delete, Submit, Next } from "$lib/components/form";
	import { FormConfig } from "$lib/config";
	import Column from "$lib/layouts/column.svelte";
	import Columns from "$lib/layouts/columns.svelte";
	import type { BudgetCategory, Budget } from "$lib/models";
	import Budgetcategories from "./budgetcategories.svelte";

    let ft = $FormConfig.formtype
    let fc = $FormConfig.collection as Budget;
    $: fc

    let budget: Budget;
    let bcatOpts: BudgetCategory[] = getCatsAsBcats(fc.id, $CategoriesStore.records);
    let section: number = 1;

    $: section
    $: bcats = fc.categories

    $: budget = {
        id: fc.id,
        month: fc.month,
        year: fc.year,
        categories: bcats
    }
</script>

{#if section == 1}
    <Fields>
        <Field name="Info">
            month: {fc.month} year: {fc.year}
        </Field>
        <Columns>
            <Column>
                {#each bcatOpts as cat}
                    {#if ( ( cat.category_id % 2 ) > 0 )}
                    <IconField item={cat}>
                        <Checkbox value={cat} fcindex={"categories"} srindex={"category_id"}/>
                    </IconField>
                    {/if}
                {/each}
            </Column>
            <Column>
                {#each bcatOpts as cat}
                    {#if ( ( cat.category_id % 2 ) == 0 )}
                    <IconField item={cat}>
                        <Checkbox value={cat} fcindex={"categories"} srindex={"category_id"}/>
                    </IconField>
                    {/if}
                {/each}
            </Column>
        </Columns>
    </Fields>

    <Buttons>
        <Cancel />
        {#if ft === 'Entry'}
            <Next bind:section/>
        {:else if ft === 'Edit'}
            <Submit />
        {:else if ft === 'Delete'}
            <Delete />
        {/if}
    </Buttons>
{:else}
    <Budgetcategories />
{/if}
