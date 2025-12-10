import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { assert } from "assertmin";
import z from "zod";

type Store = {
    state: AppState | null;
};

type AppState = {
    args: string[];
    files: {
        opened: ImageFile | null;
        folder: ImageFile[];
    }
};

const ImageFile = z.object({
    path: z.string()
});

type ImageFile = z.infer<typeof ImageFile>;

const StateUpdate = z.union([
    z.object({
        type: z.literal('set_args'),
        payload: z.array(z.string())
    }),
    z.object({
        type: z.literal('set_opened'),
        payload: ImageFile
    }),
    z.object({
        type: z.literal('extend_files'),
        payload: z.array(ImageFile)
    })
]);

export const store: Store = $state({
    state: null
});

// This module will only be invoked once so we can try to get the global state asap, and listen for updates
(async () => {
    store.state = await invoke('initial_payload');
    console.log('loaded initial payload');
})();

listen<unknown>('state_update', (event) => {
    if (store.state === null) return;

    const update = StateUpdate.parse(event.payload);

    switch (update.type) {
        case "set_args":
            store.state.args = update.payload;
            break;
        case "set_opened":
            store.state.files.opened = update.payload;
            break;
        case "extend_files":
            store.state.files.folder.push(...update.payload);
            break;
        default:
            assert.unreachable(update, "An unrecognized event was emitted from the rust side");
    }
});

