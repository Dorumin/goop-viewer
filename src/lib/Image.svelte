<script lang="ts">
    import { on } from "svelte/events";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { extensionIsVideo, getExtension } from "./utils";

    const CHECKERBOARD_BACKGROUND = true;

    type ImageProps = {
        src: string;
        alt: string;
        class: string;
        crossOrigin?: boolean;
        style?: string;
        onBitmap?: (bitmap: ImageBitmap) => void;
        onwheel?: (e: WheelEvent) => void;
        onpointerdown?: (e: PointerEvent) => void;
        onpointermove?: (e: PointerEvent) => void;
        onpointerup?: (e: PointerEvent) => void;
        onpointerleave?: (e: PointerEvent) => void;
        ondblclick?: (e: MouseEvent) => void;
        onload?: (e: Event) => void;
        onloadedmetadata?: (e: Event) => void;
        elem?: HTMLElement;
    };

    let {
        src,
        alt,
        class: cls,
        crossOrigin = false,
        style,
        onBitmap,
        onwheel,
        onpointerdown,
        onpointermove,
        onpointerup,
        onpointerleave,
        ondblclick,
        onload,
        onloadedmetadata,
        elem = $bindable()
    }: ImageProps = $props();

    const extension = $derived(getExtension(src));
    const isVideo = $derived(extensionIsVideo(extension));
    const assetSrc = $derived(src.startsWith('blob:') ? src : convertFileSrc(src));

    $effect(() => {
        // I'm scared of inconsistent dependency registration
        let _ref = elem;
        const _onBitmap = onBitmap;
        const _isBlob = src.startsWith('blob:');
        const _isVideo = isVideo;

        if (!_ref || !_onBitmap || _isBlob) return;

        if (_isVideo) {
            const video = _ref as HTMLVideoElement;
            const id = video.requestVideoFrameCallback(() => {
                createImageBitmap(video).then(_onBitmap);
            });

            return () => {
                video.cancelVideoFrameCallback(id);
            };
        } else {
            const image = _ref as HTMLImageElement;
            if (image.complete) {
                createImageBitmap(image).then(_onBitmap);
            } else {
                return on(image, 'load', () => {
                    createImageBitmap(image).then(_onBitmap);
                });
            }
        }
    });
</script>

{#if isVideo}
    <!-- svelte-ignore a11y_media_has_caption -->
    <video
        bind:this={elem}
        class={[
            "file-image video",
            cls
        ]}
        class:checkered={CHECKERBOARD_BACKGROUND}
        src={assetSrc}
        aria-label={alt}
        crossorigin={crossOrigin ? 'anonymous' : 'use-credentials'}
        style={style}
        muted
        autoplay
        loop
        {onwheel}
        {onpointerdown}
        {onpointermove}
        {onpointerup}
        {onpointerleave}
        {ondblclick}
        {onload}
        {onloadedmetadata}
        bind:this={elem}
     >
     </video>
{:else}
    <img
        bind:this={elem}
        class={[
            "file-image image",
            cls
        ]}
        class:checkered={CHECKERBOARD_BACKGROUND}
        src={assetSrc}
        alt={alt}
        crossorigin={crossOrigin ? 'anonymous' : 'use-credentials'}
        style={style}
        {onwheel}
        {onpointerdown}
        {onpointermove}
        {onpointerup}
        {onpointerleave}
        {ondblclick}
        {onload}
        {onloadedmetadata}
     >
{/if}

<style lang="scss">
    // TODO: Make this checkerboard not scale with the <img> transforms
    // Done - with var(--scale)... so hacky haha
    .checkered {
        background: repeating-conic-gradient(#8f8f8f 0 25%, #bfbfbf 0 50%) 50% / calc(20px / var(--scale)) calc(20px / var(--scale));
    }
</style>
