<script lang="ts">
    import AppNavbar from "$lib/AppNavbar.svelte";
    import Band from "$lib/Band.svelte";
    import Code from "$lib/Code.svelte";
    import Viewer from "$lib/Viewer.svelte";
    import { store } from '$lib/store.svelte';
    import terminal from 'virtual:terminal';

    // @ts-ignore It's compatible enough
    // globalThis.console = terminal;
</script>

<main class="container">
    <AppNavbar />

    {#if store.state?.files?.opened}
        <Viewer src={store.state?.files?.opened.path} />
    {/if}

    {#if store.state?.files?.folder}
        <Band files={store.state?.files?.folder} />
    {/if}

    <div class="popout">
        <div class="code-view">
            <Code>{JSON.stringify(store.state, null, 4)}</Code>
        </div>
    </div>
</main>

<style lang="scss">
    .popout {
        position: fixed;
        bottom: 0px;
        left: 0;
        width: 12px;
        height: 12px;
        background: red;
    }

    .code-view {
        display: none;
    }

    .popout:hover .code-view {
        display: block;
        position: fixed;
        bottom: 12px;
        left: 0px;
        width: 800px;
        height: 400px;
    }

    :root {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: #f6f6f6;
        background-color: rgba(0, 0, 0, 0.8);
        // Doesn't work
        // backdrop-filter: blur(5px);

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    :global(body) {
        overflow: hidden;
        margin: 0;
    }

    :global(*) {
        box-sizing: border-box;
    }

    .container {
        margin: 0;
        display: flex;
        flex-direction: column;
        text-align: center;
        height: 100vh;
        width: 100vw;
    }
</style>
