use std::{path::{Path, PathBuf}, sync::{LazyLock, RwLock}};
use serde::Serialize;

use tauri::async_runtime;

#[derive(Default, Clone, Serialize)]
pub struct AppState {
    pub config: Option<Config>,
    pub assets: Option<Assets>,
    pub cache: Option<Cache>,
    pub files: Files,
    pub args: Vec<String>,
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

pub fn default_file() -> Option<String> {
    #[cfg(debug_assertions)]
    return Some(r"D:\Images\test.jpg".to_string());

    #[cfg(not(debug_assertions))]
    return None;
}

pub fn populate_state() {
    async_runtime::spawn(async {
        let Some(file_path) = std::env::args().nth(1).or_else(default_file) else {
            return;
        };

        GLOBAL_STATE.write().unwrap().files.opened = Some(File {
            path: PathBuf::from(&file_path)
        });

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

        // files.sort_by(|a, b| natord::compare(a.path, b.path));
        // files.sort_unstable_by(|a, b|
        //     natlex_sort::nat_lex_byte_cmp(
        //         a.path.file_name().unwrap().as_encoded_bytes(),
        //         b.path.file_name().unwrap().as_encoded_bytes()
        //     )
        // );

        GLOBAL_STATE.write().unwrap().files.folder.extend(files);
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
