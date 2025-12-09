<script lang="ts">
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import GearIcon from "./GearIcon.svelte";
    import { attachHover, getFilename } from "./utils";
    import { Effect } from "@tauri-apps/api/window";
    import { store } from "./store.svelte";

    let gearHovered = $state(false);
    let effects = $state(0);

    function minimize() {
        getCurrentWebviewWindow().minimize();
    }

    async function maximize() {
        let wind = getCurrentWebviewWindow();

        if (await wind.isMaximized()) {
            wind.unmaximize();
        } else {
            wind.maximize();
        }
    }

    function close() {
        getCurrentWebviewWindow().close();
    }

    async function dragstart(e: DragEvent) {
        e.preventDefault();

        await getCurrentWebviewWindow().startDragging();
    }

    async function openSettings() {
        effects = (effects + 1) % 2;

        let newEffects: Effect[];
        switch (effects) {
            case 1: {
                newEffects = [Effect.Acrylic];
                break;
            }
            default: {
                getCurrentWebviewWindow().clearEffects();
                newEffects = [];
            }
        }

        getCurrentWebviewWindow().setEffects({
            effects: newEffects
        });
    }

    function getTitle() {
        let title = 'Goop viewer';

        if (store.state?.files?.opened) {
            let filename = getFilename(store.state.files.opened.path);

            title += ` | ${filename}`;
        }

        return title;
    }
</script>

<div class="app-navbar" draggable="true" ondragstart={dragstart} role="toolbar" aria-label="Window controls" tabindex="0">
    <div class="title">{getTitle()}</div>
    <div class="buttons">
        <button class="button minimize" title="Settings" onclick={openSettings} {@attach attachHover(h => gearHovered = h)}>
            <GearIcon animating={gearHovered} />
        </button>
        <button class="button minimize" title="Minimize" onclick={minimize}>
            <svg class="svgIcon" fill="currentColor" viewBox="0 0 50 50">
                <path d="M5,27.3v-4.6H45v4.6H5z" class="path"></path>
            </svg>
        </button>
        <button class="button maximize" title="Maximize" onclick={maximize}>
            <svg class="svgIcon" fill="currentColor" viewBox="0 0 50 50">
                <path d="M27.3,45h-4.7V5h4.7V45z M5,27.3v-4.6H45v4.6H5z" class="path"></path>
            </svg>
        </button>
        <button class="button close" title="Close" onclick={close}>
            <svg class="svgIcon" fill="currentColor" viewBox="0 0 50 50">
                <path d="M45.1,41.6l-3.3,3.3L5.1,8.2l3.3-3.3L45.1,41.6z M8.4,45.1l-3.3-3.3L41.8,5.1l3.3,3.3L8.4,45.1z" class="path"></path>
            </svg>
        </button>
    </div>
</div>

<style lang="scss">
    .app-navbar {
        display: flex;
    }

    .title {
        flex: 1;
        text-align: left;
        padding: 0px 4px;
        font-size: 12px;
    }

    .buttons {
        color: white;
        display: flex;
    }

    .button {
        border: none;
        outline: none;
        background: none;
        width: 48px;
        color: white;
    }

    .button:hover {
        background-color: rgba(255, 255, 255, 0.1);
    }

    .button svg {
        height: 12px;
        width: 12px;
    }
</style>
