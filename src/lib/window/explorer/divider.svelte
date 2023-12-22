<script lang="ts">
	import { AppState, FormState } from "$lib";
    
    let arrowShown: boolean = false;
    let disabled = $FormState

    function showarrow(visible: boolean){
        arrowShown = visible
    }
    function toggleExpanded() {
        let config = $AppState

        if (config.explorer){
            config.explorer = false
        } else {
            config.explorer = true
        }
        arrowShown = false;
        AppState.set(config)
    }
    $: normalDivider = `background: linear-gradient(0deg, rgba(0, 0, 0, 0.05) 0%, rgba(0, 0, 0, 0.05) 100%), ${$AppState.color}4d; width: 3px; margin: 5px 6px; border-radius: 1.5px;`
    $: hoverDivider = `background: linear-gradient(0deg, rgba(0, 0, 0, 0.05) 0%, rgba(0, 0, 0, 0.05) 100%), ${$AppState.color}4d; width: 15px; margin: 5px 0px; border-radius: 7.5px;`
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
{#if disabled}
    <div class="Divider" style="{hoverDivider}" />
{:else if (arrowShown && $AppState.explorer)}
    <div class="Divider" style="{hoverDivider}" on:click={toggleExpanded} on:mouseenter={()=>{showarrow(true)}} on:mouseleave={()=>{showarrow(false)}}>
        <svg xmlns="http://www.w3.org/2000/svg" width="8" height="17" viewBox="0 0 8 17" fill="none">
            <path d="M0.560626 8.50036C0.555092 8.26973 0.620505 8.03536 0.762978 7.83189L5.40183 1.20692C5.75668 0.700149 6.45515 0.576989 6.96192 0.931832C7.46869 1.28668 7.59185 1.98515 7.237 2.49192L3.03011 8.49998L7.23702 14.5081C7.59186 15.0148 7.4687 15.7133 6.96194 16.0682C6.45517 16.423 5.75669 16.2998 5.40185 15.7931L0.762996 9.1681C0.620675 8.96485 0.555248 8.73076 0.560626 8.50036Z" fill="{$AppState.color}"/>
        </svg>
    </div>
{:else if (arrowShown && $AppState.explorer == false)}
    <div class="Divider" style="{hoverDivider}" on:click={toggleExpanded} on:mouseenter={()=>{showarrow(true)}} on:mouseleave={()=>{showarrow(false)}}>
        <svg xmlns="http://www.w3.org/2000/svg" width="8" height="17" viewBox="0 0 8 17" fill="none">
            <path d="M7.43937 8.49964C7.44491 8.73027 7.37949 8.96464 7.23702 9.16811L2.59817 15.7931C2.24332 16.2999 1.54485 16.423 1.03808 16.0682C0.531312 15.7133 0.408153 15.0149 0.762996 14.5081L4.96989 8.50002L0.762978 2.49193C0.408135 1.98516 0.531295 1.28668 1.03806 0.93184C1.54483 0.576997 2.24331 0.700158 2.59815 1.20693L7.237 7.8319C7.37932 8.03515 7.44475 8.26924 7.43937 8.49964Z" fill="{$AppState.color}"/>
        </svg>
    </div>
{:else}
    <div class="Divider" style="{normalDivider}" on:click={toggleExpanded} on:mouseenter={()=>{showarrow(true)}} on:mouseleave={()=>{showarrow(false)}} />
{/if}

<style>
    .Divider {
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        justify-content: center;
        align-items: center;
        align-self: stretch;
        cursor: pointer;
    }
</style>