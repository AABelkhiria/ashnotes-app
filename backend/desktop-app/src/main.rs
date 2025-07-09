#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::Manager;
use services::github_service::{GitHubService, GitHubServiceError, Note};

struct AppState {
    github_service: Arc<GitHubService>,
}

#[tauri::command]
async fn list_notes(state: tauri::State<'_, AppState>) -> Result<Vec<Note>, GitHubServiceError> {
    state.github_service.get_all_notes().await.map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
}

#[tauri::command]
async fn get_note(state: tauri::State<'_, AppState>, path: String) -> Result<Option<Note>, GitHubServiceError> {
    state.github_service.get_note(&path).await.map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
}

#[tauri::command]
async fn create_note(state: tauri::State<'_, AppState>, path: String, content: String) -> Result<(), GitHubServiceError> {
    state.github_service.create_note(&path, &content).await
}

#[tauri::command]
async fn update_note(state: tauri::State<'_, AppState>, path: String, content: String) -> Result<(), GitHubServiceError> {
    state.github_service.update_note(&path, &content).await.map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
}

#[tauri::command]
async fn delete_note(state: tauri::State<'_, AppState>, path: String) -> Result<(), GitHubServiceError> {
    state.github_service.delete_note(&path).await.map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
}

#[tokio::main]
async fn main() {
    println!("Starting desktop application...");
    dotenvy::dotenv().ok();

    // Load environment variables for the service.
    let github_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let notes_repo_owner = std::env::var("NOTES_REPO_OWNER").expect("NOTES_REPO_OWNER must be set");
    let notes_repo_name = std::env::var("NOTES_REPO_NAME").expect("NOTES_REPO_NAME must be set");
    let app_identifier = std::env::var("APP_IDENTIFIER").unwrap_or_else(|_| "NoteApp".to_string());

    // Initialize the service here. The Tokio runtime is now active.
    let github_service = GitHubService::new(
        github_token,
        notes_repo_owner,
        notes_repo_name,
        app_identifier,
    );

    // Create and manage the application state.
    let app_state = AppState {
        github_service: Arc::new(github_service),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            list_notes,
            get_note,
            create_note,
            update_note,
            delete_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("Tauri application started.");
}
