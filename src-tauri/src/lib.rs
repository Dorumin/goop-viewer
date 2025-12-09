// use tauri::Manager;

use crate::state::populate_state;

mod commands;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // The tauri async runtime is initialized by this point
    // We can begin preloading
    populate_state();

    let builder = tauri::Builder::default()
        // .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::initial_payload,
            commands::args,
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
