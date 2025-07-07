use axum::{
    Json, Router,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod github_service;
use github_service::{GitHubService, GitHubServiceError, Note};

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../note_app_frontend/build/"]
struct Assets;

struct AppState {
    github_service: GitHubService,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let github_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let notes_repo_owner = std::env::var("NOTES_REPO_OWNER").expect("NOTES_REPO_OWNER must be set");
    let notes_repo_name = std::env::var("NOTES_REPO_NAME").expect("NOTES_REPO_NAME must be set");
    let app_identifier = std::env::var("APP_IDENTIFIER").unwrap_or_else(|_| "NoteApp".to_string());

    let github_service = GitHubService::new(github_token, notes_repo_owner, notes_repo_name, app_identifier);
    let shared_state = Arc::new(AppState { github_service });

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let app = Router::new()
        .route("/api/notes", get(list_notes).post(create_note))
        .route("/api/notes/*path", get(get_note).put(update_note).delete(delete_note))
        .fallback(static_handler)
        .with_state(shared_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.is_empty() {
        path = "index.html".to_string();
    }

    match Assets::get(path.as_str()) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data))
                .unwrap()
        }
        None => {
            match Assets::get("index.html") {
                Some(content) => {
                    let mime = mime_guess::from_path("index.html").first_or_octet_stream();
                    Response::builder()
                        .header(header::CONTENT_TYPE, mime.as_ref())
                        .body(Body::from(content.data))
                        .unwrap()
                }
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Not Found"))
                    .unwrap(),
            }
        }
    }
}

async fn list_notes(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Note>>, impl IntoResponse> {
    state
        .github_service
        .get_all_notes()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_note(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Json<Note>, impl IntoResponse> {
    match state.github_service.get_note(&path).await {
        Ok(Some(note)) => Ok(Json(note)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Note not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize)]
struct CreateNote {
    path: String,
    content: String,
}

async fn create_note(State(state): State<Arc<AppState>>, Json(payload): Json<CreateNote>) -> impl IntoResponse {
    match state.github_service.create_note(&payload.path, &payload.content).await {
        Ok(_) => (StatusCode::CREATED, "Note created".to_string()),
        Err(GitHubServiceError::NoteAlreadyExists) => {
            (StatusCode::CONFLICT, "Note with this path already exists".to_string())
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
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
        Ok(_) => (StatusCode::OK, "Note updated".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

async fn delete_note(State(state): State<Arc<AppState>>, Path(path): Path<String>) -> impl IntoResponse {
    match state.github_service.delete_note(&path).await {
        Ok(_) => (StatusCode::OK, "Note deleted".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
