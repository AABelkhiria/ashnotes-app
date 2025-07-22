use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, Path},
    http::{header, request::Parts, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use services::note_service::{Note, NoteService, NoteServiceError};

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../../frontend/build/"]
struct Assets;

struct AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let shared_state = Arc::new(AppState);

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
        None => match Assets::get("index.html") {
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
        },
    }
}

struct ApiHeaders {
    github_token: String,
    notes_repo: String,
    app_identifier: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for ApiHeaders
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let github_token = parts
            .headers
            .get("GITHUB_TOKEN")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::BAD_REQUEST, "GITHUB_TOKEN header is missing".to_string()))?;

        let notes_repo = parts
            .headers
            .get("NOTES_REPO")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::BAD_REQUEST, "NOTES_REPO header is missing".to_string()))?;

        let app_identifier = parts
            .headers
            .get("APP_IDENTIFIER")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "NoteApp".to_string());

        Ok(ApiHeaders {
            github_token,
            notes_repo,
            app_identifier,
        })
    }
}

fn get_note_service(headers: &ApiHeaders) -> Result<NoteService, NoteServiceError> {
    NoteService::new(
        headers.github_token.clone(),
        headers.notes_repo.clone(),
        headers.app_identifier.clone(),
    )
}

async fn list_notes(headers: ApiHeaders) -> Result<Json<Vec<Note>>, impl IntoResponse> {
    let note_service = get_note_service(&headers).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    note_service
        .get_all_notes()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_note(headers: ApiHeaders, Path(path): Path<String>) -> Result<Json<Note>, impl IntoResponse> {
    let note_service = get_note_service(&headers).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match note_service.get_note(&path).await {
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

async fn create_note(headers: ApiHeaders, Json(payload): Json<CreateNote>) -> impl IntoResponse {
    let note_service = match get_note_service(&headers) {
        Ok(service) => service,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    match note_service.create_note(&payload.path, &payload.content).await {
        Ok(_) => (StatusCode::CREATED, "Note created".to_string()),
        Err(NoteServiceError::NoteAlreadyExists) => {
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
    headers: ApiHeaders,
    Path(path): Path<String>,
    Json(payload): Json<UpdateNote>,
) -> impl IntoResponse {
    let note_service = match get_note_service(&headers) {
        Ok(service) => service,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    match note_service.update_note(&path, &payload.content).await {
        Ok(_) => (StatusCode::OK, "Note updated".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

async fn delete_note(headers: ApiHeaders, Path(path): Path<String>) -> impl IntoResponse {
    let note_service = match get_note_service(&headers) {
        Ok(service) => service,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    match note_service.delete_note(&path).await {
        Ok(_) => (StatusCode::OK, "Note deleted".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
