<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { store } from "./store.svelte";
	import VirtualList from 'svelte-tiny-virtual-list';
    import { getFilename } from "./utils";
    import Image from "./Image.svelte";
    import { cacheImageBitmap, openDB } from "./imagecache";

    const ITEM_SIZE = 80;
    const ITEMS_PER_SCROLL = 1;

    const { files } = $props();
    const realPaths = $derived(files.map((file: any) => convertFileSrc(file.path)));

    let listWidth = $state(400);

    let band = $state<HTMLDivElement | null>(null);

    function select(index: number) {
        store.state!.files.opened!.path = files[index].path;
    }

    function onwheel(e: WheelEvent) {
        if (e.deltaY !== 0) {
            e.preventDefault();

            // virtualScroller.scrollOffset falls out of sync with the real scrollLeft
            // And it doesn't expose a binding to the wrapper node
            const virtualScroller = band!.querySelector('.virtual-list-wrapper');
            if (virtualScroller === null) return;

            virtualScroller.scrollBy({
                // Inherit from vertical scroll
                // left: e.deltaY,
                // behavior: 'instant'
                // Scroll by fixed number of items
                left: (e.deltaY > 0 ? ITEM_SIZE : -ITEM_SIZE) * ITEMS_PER_SCROLL,
                behavior: 'smooth'
            });
            // virtualScroller.scrollOffset += e.deltaY;
            // band!.scrollLeft += e.deltaY;
        }
    }

    async function onBitmap(bitmap: ImageBitmap, src: string) {
        const db = openDB();

        try {
            await cacheImageBitmap(src, bitmap);
        } catch(e) {
            console.log('error caching', e);
        }
    }
</script>

<div class="band-container" bind:clientWidth={listWidth} onwheel={onwheel} bind:this={band}>
    <VirtualList
        itemSize={ITEM_SIZE}
        itemCount={realPaths.length}
        scrollDirection='horizontal'
        width={
            /* Literally wrong type in library */
            listWidth as unknown as string
        }
        height={ITEM_SIZE}
        overscanCount={5}
    >
        <div slot="item" let:index let:style {style}>
            <button
                class="image-container"
                class:selected={files[index].path == store.state?.files.opened?.path}
                onclick={() => select(index)}
                title={getFilename(files[index].path)}
            >
                <Image
                    src={files[index].path}
                    alt="bruh"
                    class="band-image"
                    crossOrigin={true}
                    onBitmap={bitmap => onBitmap(bitmap, files[index].path)}
                />
            </button>
        </div>
    </VirtualList>
</div>

<style lang="scss">
    // .band {
    //     width: 100%;
    //     height: 80px;
    //     display: flex;
    //     flex-direction: row;
    //     justify-content: space-around;
    //     overflow-x: auto;
    //     overflow-y: hidden;
    //     scrollbar-width: thin;
    //     scrollbar-color: black transparent;
    // }

    .band-container {
        width: 100%;
        height: 80px;
        overflow: hidden;
    }

    .band-container :global(.virtual-list-wrapper) {
        overflow-y: hidden;
        overflow-x: auto;
        scrollbar-width: thin;
        scrollbar-color: white black;
        scroll-behavior: smooth;
    }

    .image-container {
        width: 80px;
        height: 80px;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 4px;
        border-radius: 4px;
        background: none;
        border: none;
        outline: none;
        flex-shrink: 0;
        margin: 2px;
    }

    .image-container :global(.band-image) {
        max-width: 100%;
        max-height: 100%;
        width: auto;
        height: auto;
        object-fit: contain;
    }

    .image-container:hover {
        background-color: rgba(255, 255, 255, .1);
    }

    .image-container.selected {
        // Configure
        outline: 2px solid lime;
    }
</style>
