#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use services::github_service::{GitHubService, GitHubServiceError, Note};

struct AppState {
    github_service: Arc<Mutex<Option<Arc<GitHubService>>>>,
}

#[tauri::command]
fn set_credentials(
    state: tauri::State<'_, AppState>,
    github_token: String,
    notes_repo: String,
    app_identifier: String,
) -> Result<(), String> {
    let mut service_guard = state.github_service.lock().unwrap();
    let repo_parts: Vec<&str> = notes_repo.split('/').collect();
    if repo_parts.len() != 2 {
        return Err("Invalid NOTES_REPO format. Expected 'owner/name'".to_string());
    }
    let owner = repo_parts[0].to_string();
    let name = repo_parts[1].to_string();

    let new_service = Arc::new(GitHubService::new(github_token, owner, name, app_identifier));
    *service_guard = Some(new_service);
    Ok(())
}

#[tauri::command]
async fn list_notes(state: tauri::State<'_, AppState>) -> Result<Vec<Note>, GitHubServiceError> {
    let service = {
        let guard = state.github_service.lock().unwrap();
        guard.as_ref().cloned()
    };
    if let Some(service) = service {
        service.get_all_notes().await.map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
    } else {
        Err(GitHubServiceError::Anyhow("GitHub service not initialized".to_string()))
    }
}

#[tauri::command]
async fn get_note(state: tauri::State<'_, AppState>, path: String) -> Result<Option<Note>, GitHubServiceError> {
    let service = {
        let guard = state.github_service.lock().unwrap();
        guard.as_ref().cloned()
    };
    if let Some(service) = service {
        service.get_note(&path).await.map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
    } else {
        Err(GitHubServiceError::Anyhow("GitHub service not initialized".to_string()))
    }
}

#[tauri::command]
async fn create_note(state: tauri::State<'_, AppState>, path: String, content: String) -> Result<(), GitHubServiceError> {
    let service = {
        let guard = state.github_service.lock().unwrap();
        guard.as_ref().cloned()
    };
    if let Some(service) = service {
        service.create_note(&path, &content).await
    } else {
        Err(GitHubServiceError::Anyhow("GitHub service not initialized".to_string()))
    }
}

#[tauri::command]
async fn update_note(state: tauri::State<'_, AppState>, path: String, content: String) -> Result<(), GitHubServiceError> {
    let service = {
        let guard = state.github_service.lock().unwrap();
        guard.as_ref().cloned()
    };
    if let Some(service) = service {
        service.update_note(&path, &content).await.map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
    } else {
        Err(GitHubServiceError::Anyhow("GitHub service not initialized".to_string()))
    }
}

#[tauri::command]
async fn delete_note(state: tauri::State<'_, AppState>, path: String) -> Result<(), GitHubServiceError> {
    let service = {
        let guard = state.github_service.lock().unwrap();
        guard.as_ref().cloned()
    };
    if let Some(service) = service {
        service.delete_note(&path).await.map_err(|e| GitHubServiceError::Anyhow(e.to_string()))
    } else {
        Err(GitHubServiceError::Anyhow("GitHub service not initialized".to_string()))
    }
}

fn main() {
    let app_state = AppState {
        github_service: Arc::new(Mutex::new(None)),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            set_credentials,
            list_notes,
            get_note,
            create_note,
            update_note,
            delete_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
