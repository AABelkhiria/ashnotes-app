use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

mod github_service;
use github_service::{GitHubService, Note};

struct AppState {
    github_service: GitHubService,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let github_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let repo_owner = std::env::var("REPO_OWNER").expect("REPO_OWNER must be set");
    let repo_name = std::env::var("REPO_NAME").expect("REPO_NAME must be set");

    let github_service = GitHubService::new(github_token, repo_owner, repo_name);

    let shared_state = Arc::new(AppState {
        github_service,
    });

    let app = Router::new()
        .route("/api/notes", get(list_notes).post(create_note))
        .route(
            "/api/notes/:id",
            get(get_note).put(update_note).delete(delete_note),
        )
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
    Path(id): Path<String>,
) -> Result<Json<Note>, impl IntoResponse> {
    match state.github_service.get_note(&id).await {
        Ok(Some(note)) => Ok(Json(note)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Note not found")),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get note")),
    }
}

#[derive(Deserialize)]
struct CreateNote {
    filename: String,
    content: String,
}

async fn create_note(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateNote>,
) -> impl IntoResponse {
    match state.github_service.create_note(&payload.filename, &payload.content).await {
        Ok(_) => (StatusCode::CREATED, "Note created"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create note"),
    }
}

#[derive(Deserialize)]
struct UpdateNote {
    content: String,
}

async fn update_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateNote>,
) -> impl IntoResponse {
    match state.github_service.update_note(&id, &payload.content).await {
        Ok(_) => (StatusCode::OK, "Note updated"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update note"),
    }
}

async fn delete_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.github_service.delete_note(&id).await {
        Ok(_) => (StatusCode::OK, "Note deleted"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete note"),
    }
}
