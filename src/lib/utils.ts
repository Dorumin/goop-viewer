import type { Attachment } from "svelte/attachments";
import { on } from "svelte/events";

export function attachHover(cb: (hovered: boolean) => void): Attachment {
    return element => {
        const clean1 = on(element, 'pointerenter', () => cb(true));
        const clean2 = on(element, 'pointerleave', () => cb(false));

        return () => {
            clean1();
            clean2();
        };
    };
}

export function getFilename(path: string) {
    return path.replace(/.+[\\\/]/, '');
}

export function getExtension(path: string) {
    return path.replace(/.+\./, '');
}

export function extensionIsVideo(ext: string) {
    return ['webm', 'mp4', 'mkv'].includes(ext);
}
