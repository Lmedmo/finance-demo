<script lang="ts">
    import { ExplorerState, PageState, AppState, ViewState } from '$lib/config';
    
    let page: string = $PageState.title;
    let view: string = $ViewState.title;
    let expl: string = $ExplorerState.selected.name;

    let styleA: string = `color: ${$AppState.color}; font-family: ${$AppState.font};`;
    let styleB: string = `color: ${$AppState.color}; font-family: ${$AppState.font};`;
</script>

<section>
    <div class="headings">
        <h1 style="font-family: {$AppState.font};">{page}</h1>
        <h2 style="{styleA}"> ➜ View: {view}</h2>
        <h2 style="{styleB}"> ➜ Explorer Option: {expl}</h2>
    </div>
    
    <p style="font-family: {$AppState.font};">The '{view}' view is still under construction</p>
</section>
    
<style>
    section {
        display: flex;
        flex-direction: column;
        padding: 20px;
    }
    h1, h2 {
        font-size: 2.5rem;
        font-weight: 700;
    }
    .headings {
        display: flex;
        flex-direction: row;
        gap: 9px;
    }
</style>