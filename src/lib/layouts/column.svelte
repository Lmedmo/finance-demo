<div class="Column">
    <slot></slot>
</div>

<style>
    .Column {
        /* background-color: antiquewhite; */
        display: flex;
        flex-direction: column;
        align-self: stretch;
        flex: 1 0;
        gap: 15px;
    }
</style>