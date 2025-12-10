<script module lang="ts">
    export const zoom = new Tween(1, { duration: 150, easing: cubicOut });
    export const pos = new Tween({ x: 0, y: 0 }, { duration: 150, easing: cubicOut });
</script>

<script lang="ts">
    import { Tween } from 'svelte/motion';
    import { cubicOut } from 'svelte/easing';
    import assert from 'assertmin';

    import Image from "./Image.svelte";

    let imgref = $state<HTMLImageElement | HTMLVideoElement | undefined>(undefined);
    let container = $state<HTMLElement | undefined>(undefined);

    const BACKGROUND_COVER_IMAGE = true;
    const SCALE_UP_TO_VIEW = false;
    const MOVE_INSTANTLY = true;

    const { src } = $props();

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

    function getDimensions(elem: HTMLVideoElement | HTMLImageElement): [number?, number?] {
        let iw, ih;
        if (elem instanceof HTMLVideoElement) {
            iw = elem.videoWidth;
            ih = elem.videoHeight;
        } else {
            iw = elem.naturalWidth;
            ih = elem.naturalHeight;
        }

        return [iw, ih];
    }

    function onLoadedMetadata() {
        applyZoomMode(SCALE_UP_TO_VIEW ? 'contain-upscale' : 'contain');
    }

    const zoomModes = [
        "contain",
        "contain-upscale",
        "cover",
        "original"
    ] as const;
    type ZoomMode = typeof zoomModes[number];

    function applyZoomMode(scaling: ZoomMode, animate = false) {
        assert(imgref !== undefined);
        assert(container !== undefined);

        const [iw, ih] = getDimensions(imgref);

        if (!iw || !ih) return;

        const cw = container.clientWidth;
        const ch = container.clientHeight;

        let scale;

        switch (scaling) {
            case "contain":
                scale = Math.min(cw / iw, ch / ih, 1);
                break;
            case "contain-upscale":
                scale = Math.min(cw / iw, ch / ih);
                break;
            case "cover":
                scale = Math.max(cw / iw, ch / ih);
                break;
            case "original":
                scale = 1;
                break;
        }

        const cx = (cw - iw * scale) / 2;
        const cy = (ch - ih * scale) / 2;

        const options = animate ? undefined : { duration: 0 };

        zoom.set(scale, options);
        pos.set({ x: cx, y: cy }, options);
    }

    let zoomModeIndex = $state(0);

    function cycleZoomMode(e: MouseEvent) {
        e.preventDefault();

        const startZoom = zoom.target;
        const startIndex = zoomModeIndex;

        while (true) {
            applyZoomMode(zoomModes[zoomModeIndex]);

            // terminal.log(`${zoomModes[zoomModeIndex]}: ${pos.target.x} ${pos.target.y} / ${zoom.target}`);

            zoomModeIndex = (zoomModeIndex + 1) % zoomModes.length;

            if (zoom.target !== startZoom || zoomModeIndex === startIndex) {
                break;
            }
        }
    }

    function pointerDown(e: PointerEvent) {
        e.preventDefault();
        dragStart = { x: e.clientX, y: e.clientY };
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

        applyClamp();
    }

    function onWheel(e: WheelEvent) {
        e.preventDefault();

        assert(container !== undefined);

        const factor = e.deltaY < 0 ? 1.25 : 1 / 1.25;

        const rect = container.getBoundingClientRect();

        const localX = e.clientX - rect.left;
        const localY = e.clientY - rect.top;

        const currentZoom = zoom.current;
        const { x: px, y: py } = pos.current;
        const ix = (localX - px) / currentZoom;
        const iy = (localY - py) / currentZoom;

        const targetZoom = zoom.target;
        const minimumZoom = 0.05;
        const maximumZoom = 100;
        const newZoom = Math.min(maximumZoom, Math.max(minimumZoom, targetZoom * factor));
        const newX = localX - ix * newZoom;
        const newY = localY - iy * newZoom;

        const instant = MOVE_INSTANTLY ? { duration: 0 } : undefined;

        zoom.set(newZoom, instant);
        pos.set({
            x: newX,
            y: newY
        }, instant);

        // TODO: When zooming in, the clamping is not done correctly at the edges
        // Look into it, using nativeWidth/Height * zoom doesn't work in applyClamp
        applyClamp();
    }

    function applyClamp() {
        // TODO: Honestly this function might look nicer if it clamped to a square
        // instead of using the image/container's aspect ratio...
        assert(imgref !== undefined);
        assert(container !== undefined);

        const imgRect = imgref.getBoundingClientRect();
        const viewerRect = container.getBoundingClientRect();

        const ALLOWANCE = 0.5;

        // Invert the coordinate space for allowances because otherwise it's counterintuitive
        // A value of 0 would it overflow by its whole size, and 1 would keep it clamped
        // (where negative values allow it to stray further, and >1 create an inset around the container)
        // If we invert it, 0 does not let it overflow, and 1 lets it overflow, where 2 lets it go even further
        // Negatives create an inset, and 0.5 matches in both cases (the default)
        // The original inverted allowance is still useful when clamping to the container's dimensions
        const allowance = 1 - ALLOWANCE;
        const invertedAllowance = ALLOWANCE;

        let px = pos.target.x;
        let py = pos.target.y;

        const left   = imgRect.left;
        const right  = imgRect.right;
        const top    = imgRect.top;
        const bottom = imgRect.bottom;

        const imgW = imgRect.width;
        const imgH = imgRect.height;
        const vw = viewerRect.width;
        const vh = viewerRect.height;

        if (imgW <= vw) {
            // When the image is smaller than the container, we allow it to overflow by its size * -allowance
            const allowedLeft  = viewerRect.left  - imgW * -allowance;
            const allowedRight = viewerRect.right + imgW * -allowance;

            if (left > allowedRight) {
                px -= (left - allowedRight);
            } else if (right < allowedLeft) {
                px += (allowedLeft - right);
            }
        } else {
            // When the image is larger than container, we allow it to overflow by half of the viewer's dimension
            // Fortuitously, the position is in screen space (relative to container), so
            // this makes this clamping easy without taking into account scaling
            const viewerAllowance = vw * invertedAllowance;
            const allowedLeft   = viewerRect.left + viewerAllowance;
            const allowedRight  = viewerRect.right - viewerAllowance;

            if (left > allowedLeft) {
                px -= (left - allowedLeft);
            } else if (right < allowedRight) {
                px += (allowedRight - right);
            }
        }

        // Same cases as above but for the vertical dimension
        if (imgH <= vh) {
            const allowedTop    = viewerRect.top    - imgH * -allowance;
            const allowedBottom = viewerRect.bottom + imgH * -allowance;

            if (top > allowedBottom) {
                py -= (top - allowedBottom);
            } else if (bottom < allowedTop) {
                py += (allowedTop - bottom);
            }
        } else {
            const viewerAllowance = vh * invertedAllowance;
            const allowedTop    = viewerRect.top + viewerAllowance;
            const allowedBottom = viewerRect.bottom - viewerAllowance;

            if (top > allowedTop) {
                py -= (top - allowedTop);
            } else if (bottom < allowedBottom) {
                py += (allowedBottom - bottom);
            }
        }

        if (px !== pos.target.x || py !== pos.target.y) {
            pos.set({ x: px, y: py });
        }
    }
</script>

<!-- Bind leave and move events to the container which covers the area "outside" the image -->
<div
    class="viewer-container"
    bind:this={container}
    onpointerleave={pointerUp}
    onpointermove={pointerMove}
    onpointerdown={pointerDown}
    onpointerup={pointerUp}
    onwheel={onWheel}
>
    <Image
        {src}
        alt={src}
        class="viewer-image"
        crossOrigin
        style="
            --scale: {zoom.current};
            transform:
                translate({pos.current.x}px, {pos.current.y}px)
                scale({zoom.current})
        "
        onload={onLoadedMetadata}
        onloadedmetadata={onLoadedMetadata}
        ondblclick={cycleZoomMode}
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
        // TODO: Make configurable, but this is the best default
        // Maybe even pixelated when >100%, smooth when <100%?
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
