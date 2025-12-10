use std::time::Instant;

// use tauri::Manager;
use tauri::Emitter;

use crate::state::{GLOBAL_APP_HANDLE, populate_state};

mod commands;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let start = Instant::now();
    // The tauri async runtime is initialized by this point
    // We can begin preloading
    populate_state();

    let builder = tauri::Builder::default()
        // .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::initial_payload,
            commands::args,
        ])
        .setup(move |app| {
            GLOBAL_APP_HANDLE.set(app.handle().clone()).expect("app does not get initialized more than once");

            eprintln!("delay from start: {:?}", start.elapsed());
            app.emit("setup-test-event", "App setup is running").unwrap();

            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
