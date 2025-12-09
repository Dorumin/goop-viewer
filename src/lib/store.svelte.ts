type Store = {
    state: AppState | null;
};

type AppState = {
    files: {
        opened: ImageFile | null;
        folder: ImageFile[];
    }
};

type ImageFile = {
    path: string;
}

export const store: Store = $state({
    state: null
});
