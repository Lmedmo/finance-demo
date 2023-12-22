<script>
    import { AppState } from "$lib/config";
</script>

{#if $AppState.os === 'darwin'}
    <div class="MacCluster">
        <slot></slot>
    </div>
{:else if $AppState.os === 'win32'}
    <div class="WinCluster">
        <slot></slot>
    </div>
{/if}

<style>
    div {
        display: flex;
        box-sizing: border-box;
        align-items: stretch;
        justify-content: center;
        align-self: center;
        gap: 20px;
        /* padding-left: 25px; */
        height: 30%;
        width: 100%;
        max-width: 1700px;
        padding-left: 25px;
    }
    .MacCluster {
        max-height: 400px;
        min-height: 300px;
    }
    .WinCluster {
        max-height: 350px;
        min-height: 250px;
    }
</style>