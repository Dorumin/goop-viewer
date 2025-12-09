import assert from 'assertmin';

const DB_NAME = 'goop-viewer-image-cache-db';
const STORE_NAME = 'previews';
const DB_VERSION = 1;

let dbPromise: Promise<IDBDatabase>;

export function openDB() {
    if (!dbPromise) {
        dbPromise = new Promise((resolve, reject) => {
            const req = indexedDB.open(DB_NAME, DB_VERSION);

            req.onupgradeneeded = () => {
                const db = req.result;
                if (!db.objectStoreNames.contains(STORE_NAME)) {
                    db.createObjectStore(STORE_NAME);
                }
            };

            req.onsuccess = () => resolve(req.result);
            req.onerror = () => reject(req.error);
        });
    }

    return dbPromise;
}

export function putImageBlob(key: string, blob: Blob) {
    return new Promise(async (resolve, reject) => {
        const db = await openDB();
        const tx = db.transaction(STORE_NAME, 'readwrite');
        const store = tx.objectStore(STORE_NAME);

        const req = store.put(blob, key);
        req.onsuccess = () => resolve(undefined);
        req.onerror = () => reject(req.error);
    });
}

export function getImageBlob(key: string): Promise<Blob> {
    return new Promise(async (resolve, reject) => {
        const db = await openDB();
        const tx = db.transaction(STORE_NAME, 'readonly');
        const store = tx.objectStore(STORE_NAME);

        const req = store.get(key);
        req.onsuccess = () => resolve(req.result || null);
        req.onerror = () => reject(req.error);
    });
}

function blobToUrl(blob: Blob) {
    return URL.createObjectURL(blob);
}

async function imageBitmapToPngBlob(bitmap: ImageBitmap) {
    const off = new OffscreenCanvas(bitmap.width, bitmap.height);
    const ctx = off.getContext('2d');
    assert(ctx !== null);
    ctx.drawImage(bitmap, 0, 0);

    return await off.convertToBlob({ type: 'image/png' });
}

export async function cacheImageBitmap(key: string, bitmap: ImageBitmap) {
    const blob = await imageBitmapToPngBlob(bitmap);
    await putImageBlob(key, blob);

    return blobToUrl(blob);
}

export async function getCachedImageUrl(key: string) {
    const blob = await getImageBlob(key);

    return blob ? blobToUrl(blob) : null;
}

export function drawBitmapToCanvas(canvas: HTMLCanvasElement, imageBitmap: ImageBitmap) {
    const ctx = canvas.getContext('2d');
    assert(ctx !== null);
    ctx.drawImage(imageBitmap, 0, 0);
}
