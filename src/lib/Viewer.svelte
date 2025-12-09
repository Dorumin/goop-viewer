<script lang="ts">
    import { Tween } from 'svelte/motion';
    import { cubicOut } from 'svelte/easing';
    import assert from 'assertmin';

    import Image from "./Image.svelte";

    let imgref = $state<HTMLImageElement | HTMLVideoElement | undefined>(undefined);
    let container = $state<HTMLElement | undefined>(undefined);

    const BACKGROUND_COVER_IMAGE = true;
    const SCALE_UP_TO_VIEW = false;

    const { src } = $props();

    const zoom = new Tween(1, { duration: 150, easing: cubicOut });
    const pos = new Tween({ x: 0, y: 0 }, { duration: 150, easing: cubicOut });

    let dragStart = $state<{ x: number, y: number } | null>(null);

    $effect(() => {
        src;

        zoom.set(0, {
            duration: 0
        });

        pos.set({ x: 0, y: 0 }, {
            duration: 0
        });

        setTimeout(() => {
            // If by the next frame it still hasn't loaded the file metadata, then leave it default
            if (zoom.target == 0) {
                zoom.set(1, {
                    duration: 0
                });
            }
        }, 50);
    });

    function onLoadedMetadata() {
        assert(imgref !== undefined);
        assert(container !== undefined);

        let iw, ih;
        if (imgref instanceof HTMLVideoElement) {
            iw = imgref.videoWidth;
            ih = imgref.videoHeight;
        } else {
            iw = imgref.naturalWidth;
            ih = imgref.naturalHeight;
        }

        console.log({ iw, ih });

        if (!iw || !ih) return;

        // Container size
        const cw = container.clientWidth;
        const ch = container.clientHeight;

        // Fit image to container (just like object-fit: contain)
        const scaleX = cw / iw;
        const scaleY = ch / ih;
        const fitScale = SCALE_UP_TO_VIEW
            ? Math.min(scaleX, scaleY)
            : Math.min(scaleX, scaleY, 1);

        // Center the positioned image
        const centeredX = (cw - iw * fitScale) / 2;
        const centeredY = (ch - ih * fitScale) / 2;

        // Apply immediately (no animation)
        zoom.set(fitScale, { duration: 0 });
        pos.set({ x: centeredX, y: centeredY }, { duration: 0 });
    }

    function pointerDown(e: PointerEvent) {
        e.preventDefault();
        dragStart = { x: e.clientX, y: e.clientY };
        console.log(dragStart);
    }

    function pointerMove(e: PointerEvent) {
        if (!dragStart) return;
        const dx = e.clientX - dragStart.x;
        const dy = e.clientY - dragStart.y;
        dragStart = {
            x: e.clientX,
            y: e.clientY
        };

        pos.set(({
            x: pos.target.x + dx,
            y: pos.target.y + dy
        }), {
            // When dragging, skip the animation, just make it instant
            duration: 0
        });
    }

    function pointerUp() {
        dragStart = null;
    }

    function onWheel(e: WheelEvent) {
        e.preventDefault();

        const factor = e.deltaY < 0 ? 1.25 : 1 / 1.25;

        const currentZoom = zoom.current;
        const { x: px, y: py } = pos.current;
        const ix = (e.clientX - px) / currentZoom;
        const iy = (e.clientY - py) / currentZoom;

        const targetZoom = zoom.target;
        const minimumZoom = 0.05;
        const newZoom = Math.max(targetZoom * factor, minimumZoom);
        zoom.set(newZoom);

        pos.set({
            x: e.clientX - ix * newZoom,
            y: e.clientY - iy * newZoom
        });
    }
</script>

<!-- Bind leave and move events to the container which covers the area "outside" the image -->
<div class="viewer-container" bind:this={container} onpointerleave={pointerUp} onpointermove={pointerMove}>
    <Image
        {src}
        alt={src}
        class="viewer-image"
        crossOrigin
        style="
            transform:
                translate({pos.current.x}px, {pos.current.y}px)
                scale({zoom.current});
        "
        onwheel={onWheel}
        onpointerdown={pointerDown}
        onpointerup={pointerUp}
        onload={onLoadedMetadata}
        onloadedmetadata={onLoadedMetadata}
        bind:elem={imgref}
    />

    {#if BACKGROUND_COVER_IMAGE}
        <div class="background-cover">
            <Image
                {src}
                alt={src}
                class="background-cover-image"
                crossOrigin
            />
            <!-- <img class="background-cover-image" src={realSrc} alt={realSrc}> -->
        </div>
    {/if}
</div>

<style lang="scss">
    .viewer-container {
        display: flex;
        // Don't center when manually managing position/zoom
        justify-content: start;
        align-items: start;
        overflow: hidden;
        flex: 1;
    }

    .viewer-container :global(.viewer-image) {
        // max-height: 100%;
        // max-width: 100%;

        // width: auto;
        // height: auto;

        // object-fit: contain;

        // TODO: Make configurable, but this is the best default
        image-rendering: pixelated;

        user-select: none;
        transform-origin: top left;
    }

    .background-cover {
        position: fixed;
        top: -10px;
        left: -10px;
        right: -10px;
        bottom: -10px;
        padding: 10px;
        filter: blur(10px) brightness(0.3);
        // opacity: 0.5;
        z-index: -1;
    }

    .background-cover :global(.background-cover-image) {
        min-height: 100%;
        min-width: 100%;
        width: auto;
        height: auto;
        transform: translate(-50%, -50%);
        top: 50%;
        left: 50%;
        position: absolute;
    }
</style>
