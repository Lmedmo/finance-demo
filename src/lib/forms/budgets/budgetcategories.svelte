<script lang="ts">
	import { BudgetsStore, Selector, Buttons, Cancel, Delete, IconField, Fields, FormConfig, Submit, Trash } from "$lib";
    import type { Budget, BudgetCategory, BudgetType } from '$lib/models'

    let fc = $FormConfig.collection as Budget
    let ft = $FormConfig.formtype

    $: fc

    let bcats = fc.categories
    //let bcat: BudgetCategory
    //let calc_type: BudgetType = fc;

    //$: calc_type

    function setAttribute(c: BudgetCategory){
        //console.log("Calc_type: ", calc_type)
        console.log("cat: ", c)
    }

    // $: bcat {
    //     category_id: cat.id,
    //     name: cat.name,
    //     budget_id: bid, 
    //     calc_type: 1, 
    //     icon: fc.icon, 
    //     icon_color: fc.categories[].icon_color
    // }
</script>

<Fields>
    {#each bcats as cat}
    <IconField item={cat}>
        <!-- <Selector store={$BudgetsStore.types} bind:selected={calc_type} on:optSet={()=>setAttribute(cat)} /> -->
        {cat}
    </IconField>
    {/each}
</Fields>

<Buttons>
    <Cancel />
    {#if ft === 'Entry'}
        <Submit/>
    {:else if ft === 'Edit'}
        <Submit />
        <Trash />
    {:else if ft === 'Delete'}
        <Delete />
    {/if}
</Buttons>
<!-- {#each  as }
    
{/each} -->