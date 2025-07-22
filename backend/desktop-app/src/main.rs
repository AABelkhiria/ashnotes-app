#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use services::github_service::{GitHubService, GitHubServiceError, Note};
use std::sync::Mutex;

use log::{error, info};
use tauri_plugin_log::TimezoneStrategy;

// --- State Management ---
struct AppState {
    github_token: Mutex<Option<String>>,
    notes_repo: Mutex<Option<String>>,
    app_identifier: Mutex<Option<String>>,
    initialized: Mutex<bool>,
}

impl AppState {
    /// Creates a GitHubService instance from the current state.
    fn get_service(&self) -> Result<GitHubService, GitHubServiceError> {
        info!("Creating GitHubService from state");
        // 1. Lock mutexes and extract credentials.
        let github_token = self
            .github_token
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| GitHubServiceError::Anyhow("GitHub token not set".to_string()))?;

        let notes_repo = self
            .notes_repo
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| GitHubServiceError::Anyhow("Notes repo not set".to_string()))?;

        let app_identifier = self
            .app_identifier
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_else(|| "NoteApp".to_string());

        // 2. Parse the repo string.
        let repo_parts: Vec<&str> = notes_repo.split('/').collect();
        if repo_parts.len() != 2 {
            return Err(GitHubServiceError::Anyhow(
                "Invalid NOTES_REPO format. Expected 'owner/name'".to_string(),
            ));
        }
        let owner = repo_parts[0].to_string();
        let name = repo_parts[1].to_string();

        // 3. Create and return the service.
        Ok(GitHubService::new(github_token, owner, name, app_identifier))
    }
}

#[tauri::command]
fn is_initialized(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    info!("Checking if the app is initialized");
    Ok(*state.initialized.lock().unwrap())
}

#[tauri::command]
fn set_credentials(
    state: tauri::State<'_, AppState>,
    github_token: String,
    notes_repo: String,
    app_identifier: String,
) -> Result<(), String> {
    info!("Setting credentials: ***, {}, {}", notes_repo, app_identifier);
    let mut initialized = state.initialized.lock().unwrap();
    *state.github_token.lock().unwrap() = Some(github_token);
    *state.notes_repo.lock().unwrap() = Some(notes_repo);
    *state.app_identifier.lock().unwrap() = Some(app_identifier);
    *initialized = true;
    Ok(())
}

#[tauri::command]
async fn list_notes(state: tauri::State<'_, AppState>) -> Result<Vec<Note>, GitHubServiceError> {
    info!("Listing all notes");
    let service = state.get_service()?;
    info!("Fetching all notes from service");
    service
        .get_all_notes()
        .await
        .map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
}

#[tauri::command]
async fn get_note(state: tauri::State<'_, AppState>, path: String) -> Result<Option<Note>, GitHubServiceError> {
    info!("Getting note at path: {}", path);
    let service = state.get_service()?;
    service
        .get_note(&path)
        .await
        .map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
}

#[tauri::command]
async fn create_note(
    state: tauri::State<'_, AppState>,
    path: String,
    content: String,
) -> Result<(), GitHubServiceError> {
    info!("Creating note at path: {}", path);
    let service = state.get_service()?;
    service
        .create_note(&path, &content)
        .await
        .map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
}

#[tauri::command]
async fn update_note(
    state: tauri::State<'_, AppState>,
    path: String,
    content: String,
) -> Result<(), GitHubServiceError> {
    info!("Updating note at path: {}", path);
    let service = state.get_service()?;
    service
        .update_note(&path, &content)
        .await
        .map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
}

#[tauri::command]
async fn delete_note(state: tauri::State<'_, AppState>, path: String) -> Result<(), GitHubServiceError> {
    info!("Deleting note at path: {}", path);
    let service = state.get_service()?;
    service
        .delete_note(&path)
        .await
        .map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
}

#[tauri::command]
fn log_message(message: String) {
    info!("[Frontend] {}", message);
}

// --- Main Function (unchanged) ---
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
                // .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .level(log::LevelFilter::Info)
                .level_for("octocrab", log::LevelFilter::Info)
                .level_for("hyper", log::LevelFilter::Info)
                .level_for("rustls", log::LevelFilter::Info)
                .level_for("hyper_rustls", log::LevelFilter::Info)
                .level_for("tower", log::LevelFilter::Info)
                .level_for("hyper_util", log::LevelFilter::Info)
                .build(),
        )
        .setup(|_app| {
            // Log the current executable path
            match std::env::current_exe() {
                Ok(exe_path) => info!("Current executable path: {:?}", exe_path),
                Err(e) => error!("Failed to get current executable path: {}", e),
            }
            // Log the current working directory
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
