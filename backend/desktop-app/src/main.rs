#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use log::{error, info};
use tauri_plugin_log::TimezoneStrategy;

mod commands;
mod state;
mod tray;

use commands::{
    create_note, delete_note, get_note, is_initialized, list_notes, log_message, set_credentials, update_note,
};
use state::AppState;

fn main() {
    let app_state = AppState {
        github_token: Mutex::new(None),
        notes_repo: Mutex::new(None),
        app_identifier: Mutex::new(None),
        initialized: Mutex::new(false),
    };

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .level(
                    if std::env::var("DEBUG_BUILD").is_ok() && std::env::var("DEBUG_BUILD").unwrap_or_default() != "0" {
                        log::LevelFilter::Info
                    } else {
                        log::LevelFilter::Off
                    },
                )
                .level_for("octocrab", log::LevelFilter::Info)
                .level_for("hyper", log::LevelFilter::Info)
                .level_for("rustls", log::LevelFilter::Info)
                .level_for("hyper_rustls", log::LevelFilter::Info)
                .level_for("tower", log::LevelFilter::Info)
                .level_for("hyper_util", log::LevelFilter::Info)
                .build(),
        )
        .setup(|_app| {
            tray::create_tray(_app.handle())?;

            match std::env::current_exe() {
                Ok(exe_path) => info!("Current executable path: {:?}", exe_path),
                Err(e) => error!("Failed to get current executable path: {}", e),
            }
            match std::env::current_dir() {
                Ok(current_dir) => info!("Current working directory: {:?}", current_dir),
                Err(e) => error!("Failed to get current working directory: {}", e),
            }
            info!("Tauri server v{} is starting up", env!("CARGO_PKG_VERSION"));
            Ok(())
        })
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            set_credentials,
            is_initialized,
            list_notes,
            get_note,
            create_note,
            update_note,
            delete_note,
            log_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
