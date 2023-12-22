<div class="content">
    <slot></slot>
</div>

<style>
.content {
    /* background-color: beige; */
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    align-items: center;
    justify-content: flex-start;
    align-self: stretch;
    flex: 1 0;
    padding: 0px 30px 30px 30px;
    overflow-y: auto;
    overflow-x: hidden;
}
</style>