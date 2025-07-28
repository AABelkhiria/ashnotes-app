use super::state::AppState;
use log::info;
use serde::Serialize;
use services::note_service::{CreateNote, Note, NoteServiceError, UpdateNote};

#[derive(Debug, Serialize)]
pub enum TauriError {
    NoteAlreadyExists,
    GitHub(String),
    Anyhow(String),
}

impl From<anyhow::Error> for TauriError {
    fn from(err: anyhow::Error) -> Self {
        TauriError::Anyhow(err.to_string())
    }
}

impl From<NoteServiceError> for TauriError {
    fn from(err: NoteServiceError) -> Self {
        match err {
            NoteServiceError::NoteAlreadyExists => TauriError::NoteAlreadyExists,
            NoteServiceError::GitHub(s) => TauriError::GitHub(s),
            NoteServiceError::Anyhow(s) => TauriError::Anyhow(s),
        }
    }
}

#[tauri::command]
pub fn is_initialized(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    info!("Checking if the app is initialized");
    Ok(*state.initialized.lock().unwrap())
}

#[tauri::command]
pub fn set_credentials(
    state: tauri::State<'_, AppState>,
    github_token: String,
    notes_repo: String,
    app_identifier: String,
) -> Result<(), String> {
    info!("Setting credentials: {}, {}", notes_repo, app_identifier);
    let mut initialized = state.initialized.lock().unwrap();
    *state.github_token.lock().unwrap() = Some(github_token);
    *state.notes_repo.lock().unwrap() = Some(notes_repo);
    *state.app_identifier.lock().unwrap() = Some(app_identifier);
    *initialized = true;
    Ok(())
}

#[tauri::command]
pub async fn list_notes(state: tauri::State<'_, AppState>) -> Result<Vec<Note>, TauriError> {
    info!("Listing all notes");
    let service = state.get_service()?;
    info!("Fetching all notes from service");
    service.get_all_notes().await.map_err(Into::into)
}

#[tauri::command]
pub async fn get_note(state: tauri::State<'_, AppState>, path: String) -> Result<Option<Note>, TauriError> {
    info!("Getting note at path: {}", path);
    let service = state.get_service()?;
    service.get_note(&path).await.map_err(Into::into)
}

#[tauri::command]
pub async fn create_note(state: tauri::State<'_, AppState>, payload: CreateNote) -> Result<(), TauriError> {
    info!("Creating note at path: {}", payload.path);
    let service = state.get_service()?;
    service.create_note(&payload).await.map_err(Into::into)
}

#[tauri::command]
pub async fn update_note(
    state: tauri::State<'_, AppState>,
    path: String,
    payload: UpdateNote,
) -> Result<(), TauriError> {
    info!("Updating note at path: {}", path);
    let service = state.get_service()?;
    service.update_note(&path, &payload).await.map_err(Into::into)
}

#[tauri::command]
pub async fn delete_note(state: tauri::State<'_, AppState>, path: String) -> Result<(), TauriError> {
    info!("Deleting note at path: {}", path);
    let service = state.get_service()?;
    service.delete_note(&path).await.map_err(Into::into)
}

#[tauri::command]
pub fn log_message(message: String) {
    info!("[Frontend] {}", message);
}
