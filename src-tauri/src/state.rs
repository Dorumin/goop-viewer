use std::{path::{Path, PathBuf}, sync::{LazyLock, OnceLock, RwLock}, time::Duration};
use serde::Serialize;

use tauri::{AppHandle, Emitter, async_runtime};

#[derive(Serialize, Clone)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
enum StateUpdate {
    SetArgs(Vec<String>),
    SetOpened(File),
    ExtendFiles(Vec<File>)
}

#[derive(Default, Clone, Serialize)]
pub struct AppState {
    pub config: Option<Config>,
    pub assets: Option<Assets>,
    pub cache: Option<Cache>,
    pub files: Files,
    pub args: Vec<String>,
}

impl AppState {
    pub fn update(&mut self, update: StateUpdate) {
        if let Some(handle) = GLOBAL_APP_HANDLE.get() {
            eprintln!("emitting state_update event");
            handle.emit("state_update", &update).unwrap();
        };

        match update {
            StateUpdate::SetArgs(args) => {
                self.args = args;
            }
            StateUpdate::SetOpened(opened) => {
                self.files.opened = Some(opened);
            }
            StateUpdate::ExtendFiles(files) => {
                self.files.folder.extend(files);
            },
        }
    }

    pub fn set_args(&mut self, args: Vec<String>) {
        if let Some(handle) = GLOBAL_APP_HANDLE.get() {
            eprintln!("emitting set_args event");
            handle.emit("set_args", &args).unwrap();
            handle.emit("state_update", StateUpdate::SetArgs(vec![])).unwrap();
        };

        self.args = args;
    }

    pub fn extend_files(&mut self, files: Vec<File>) {
        GLOBAL_APP_HANDLE.get().map(|handle| {
            eprintln!("emitting extend_files event");
            handle.emit("extend_files", &files)
        });

        self.files.folder.extend(files);
    }
}

#[derive(Default, Clone, Serialize)]
pub struct Config {}

#[derive(Default, Clone, Serialize)]
pub struct Assets {}

#[derive(Default, Clone, Serialize)]
pub struct Cache {}

#[derive(Default, Clone, Serialize)]
pub struct Files {
    opened: Option<File>,
    folder: Vec<File>
}

#[derive(Default, Clone, Serialize)]
pub struct File {
    path: PathBuf
}

pub static GLOBAL_STATE: LazyLock<RwLock<AppState>> = LazyLock::new(|| RwLock::new(AppState::default()));
pub static GLOBAL_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn default_file() -> Option<String> {
    #[cfg(debug_assertions)]
    return Some(r"D:\Images\test.jpg".to_string());

    #[cfg(not(debug_assertions))]
    return None;
}

pub fn populate_state() {
    async_runtime::spawn(async {
        let args: Vec<_> = std::env::args().collect();
        let file_path = args.get(1).cloned().or_else(default_file);

        GLOBAL_STATE.write().unwrap().update(StateUpdate::SetArgs(args));

        let Some(file_path) = file_path else {
            return;
        };

        GLOBAL_STATE.write().unwrap().update(StateUpdate::SetOpened(File {
            path: PathBuf::from(&file_path)
        }));

        let path = PathBuf::from(file_path);
        let Some(directory) = path.parent() else {
            return;
        };

        let Ok(mut children) = tokio::fs::read_dir(directory).await else {
            return;
        };

        let mut files = vec![];

        while let Ok(Some(child)) = children.next_entry().await {
            let path = child.path();

            if is_accepted_path(&path) {
                files.push(File {
                    path
                });
            }
        }

        tokio::time::sleep(Duration::from_secs(2)).await;

        // files.sort_by(|a, b| natord::compare(a.path, b.path));
        // files.sort_unstable_by(|a, b|
        //     natlex_sort::nat_lex_byte_cmp(
        //         a.path.file_name().unwrap().as_encoded_bytes(),
        //         b.path.file_name().unwrap().as_encoded_bytes()
        //     )
        // );


        GLOBAL_STATE.write().unwrap().update(StateUpdate::ExtendFiles(files));
    });
}

pub fn is_accepted_path(path: &Path) -> bool {
    let extension = path.extension().map(|s| s.to_string_lossy());

    let accepted_extensions = ["jpg", "jpeg", "png", "webp", "gif", "avif", "webm", "mp4", "mkv"];

    if let Some(extension) = extension {
        if accepted_extensions.iter().any(|ext| *ext == extension) {
            return true;
        }
    }

    false
}
