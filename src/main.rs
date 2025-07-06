use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use std::sync::Arc;

mod github_service;
use github_service::{GitHubService, Note, GitHubServiceError};

struct AppState {
    github_service: GitHubService,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let github_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let notes_repo_owner = std::env::var("NOTES_REPO_OWNER").expect("NOTES_REPO_OWNER must be set");
    let notes_repo_name = std::env::var("NOTES_REPO_NAME").expect("NOTES_REPO_NAME must be set");

    let app_identifier = std::env::var("APP_IDENTIFIER").unwrap_or_else(|_| "Ash".to_string());

    let github_service = GitHubService::new(github_token, notes_repo_owner, notes_repo_name, app_identifier);

    let shared_state = Arc::new(AppState { github_service });

    let app = Router::new()
        .route("/api/notes", get(list_notes).post(create_note))
        .route("/api/notes/*path", get(get_note).put(update_note).delete(delete_note))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn list_notes(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Note>>, impl IntoResponse> {
    match state.github_service.get_all_notes().await {
        Ok(notes) => Ok(Json(notes)),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get notes")),
    }
}

async fn get_note(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Json<Note>, impl IntoResponse> {
    match state.github_service.get_note(&path).await {
        Ok(Some(note)) => Ok(Json(note)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Note not found")),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get note")),
    }
}

#[derive(Deserialize)]
struct CreateNote {
    path: String,
    content: String,
}

async fn create_note(State(state): State<Arc<AppState>>, Json(payload): Json<CreateNote>) -> impl IntoResponse {
    match state.github_service.create_note(&payload.path, &payload.content).await {
        Ok(_) => (StatusCode::CREATED, "Note created"),
        Err(GitHubServiceError::NoteAlreadyExists) => (StatusCode::CONFLICT, "Note with this path already exists"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create note"),
    }
}

#[derive(Deserialize)]
struct UpdateNote {
    content: String,
}

async fn update_note(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    Json(payload): Json<UpdateNote>,
) -> impl IntoResponse {
    match state.github_service.update_note(&path, &payload.content).await {
        Ok(_) => (StatusCode::OK, "Note updated"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update note"),
    }
}

async fn delete_note(State(state): State<Arc<AppState>>, Path(path): Path<String>) -> impl IntoResponse {
    match state.github_service.delete_note(&path).await {
        Ok(_) => (StatusCode::OK, "Note deleted"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete note"),
    }
}
