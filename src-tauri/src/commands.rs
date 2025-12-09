use crate::state::{AppState, GLOBAL_STATE};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn initial_payload() -> AppState {
    GLOBAL_STATE.read().unwrap().clone()
}

#[tauri::command]
pub fn args() -> Vec<String> {
    std::env::args().collect()
}
