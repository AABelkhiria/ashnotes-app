use services::note_service::{NoteService, NoteServiceError};
use std::sync::Mutex;
use log::info;

// --- State Management ---
pub struct AppState {
    pub github_token: Mutex<Option<String>>,
    pub notes_repo: Mutex<Option<String>>,
    pub app_identifier: Mutex<Option<String>>,
    pub initialized: Mutex<bool>,
}

impl AppState {
    /// Creates a NoteService instance from the current state.
    pub fn get_service(&self) -> Result<NoteService, NoteServiceError> {
        info!("Creating NoteService from state");
        // 1. Lock mutexes and extract credentials.
        let github_token = self
            .github_token
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| NoteServiceError::Anyhow("GitHub token not set".to_string()))?;

        let notes_repo = self
            .notes_repo
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| NoteServiceError::Anyhow("Notes repo not set".to_string()))?;

        let app_identifier = self
            .app_identifier
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_else(|| "NoteApp".to_string());

        // 2. Create and return the service.
        NoteService::new(github_token, notes_repo, app_identifier)
    }
}
