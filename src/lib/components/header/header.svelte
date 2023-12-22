<script lang="ts">
    import { AppState, ViewState, PageState } from "$lib/config";
    import type { ExplOption } from "$lib/config";
    import type { User } from '$lib'

    export let src: User | ExplOption;

    $: style = `color: ${$AppState.text}; font-family: ${$AppState.font}; text-shadow: 0px 0px ${$AppState.text};`;
    $: iconstyle = `background: ${src.icon_color};`
    $: currentPageID = $PageState.id
</script>

{#if (currentPageID === 0)}
    {#if $AppState.os === 'darwin'}
    <div {style} class="MacHeader">
        <div class="MacIcon"><img class="Macimg" src="{src.icon}" alt="icon"></div>
        {$ViewState.title}
    </div>
    {:else if $AppState.os === 'win32'}
    <div {style} class="WinHeader">
        <div class="WinIcon"><img class="Winimg" src="{src.icon}" alt="icon"></div>
        {$ViewState.title}
    </div>
    {/if}
{:else}
    {#if $AppState.os === 'darwin'}
        <div {style} class="MacHeader">
            <div class="MacIcon" style="{iconstyle}"><img class="Macimg" src="{src.icon}" alt="icon"></div>
            {src.name}
        </div>
    {:else if $AppState.os === 'win32'}
        <div {style} class="WinHeader">
            <div class="WinIcon" style="{iconstyle}"><img class="Winimg" src="{src.icon}" alt="icon"></div>
            {src.name}
        </div>
    {/if}
{/if}


<style>
    div {
        display: flex;
        flex-direction: row;
        box-sizing: border-box;
    }
/* Mac */
    .MacHeader {
        gap: 20px;
        align-self: center;
        align-items: center;
        justify-content: flex-start;
        width: 100%;
        padding: 40px 0px 40px 25px;
        max-width: 1700px;
        font-size: 2.5rem;
        font-weight: bold;
    }
    .MacIcon {
        height: 90px;
        width: 90px;
        background-color: black;
        border-radius: 22px;
    }
    .Macimg {
        height: 90px;
        width: 90px;
        border-radius: 22px;
    }
/* Windows */
    .WinHeader {
        gap: 20px;
        align-self: center;
        align-items: center;
        justify-content: flex-start;
        width: 100%;
        padding: 40px 0px 40px 25px;
        max-width: 1700px;
        font-size: 2rem;
        font-weight: bold;
    }
    .WinIcon {
        height: 65px;
        width: 65px;
        background-color: black;
        border-radius: 16px;
    }
    .Winimg {
        height: 65px;
        width: 65px;
        border-radius: 16px;
    }
</style>