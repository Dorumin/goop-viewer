<script lang="ts">
    const { children } = $props();
</script>

<pre class="codeblock">{@render children()}</pre>

<style lang="scss">
    .codeblock {
        background: #222;
        border: 1px solid #ccc;
        border-radius: 6px;
        padding: 4px;
        text-align: left;
        max-height: 100%;
        overflow-y: auto;
        margin: 0;
    }
</style>
