use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json,
};
use services::note_service::{CreateNote, Note, NoteService, NoteServiceError, UpdateNote};

pub struct ApiHeaders {
    pub github_token: String,
    pub notes_repo: String,
    pub app_identifier: String,
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

pub fn get_note_service(headers: &ApiHeaders) -> Result<NoteService, NoteServiceError> {
    NoteService::new(
        headers.github_token.clone(),
        headers.notes_repo.clone(),
        headers.app_identifier.clone(),
    )
}

pub async fn list_notes(headers: ApiHeaders) -> Result<Json<Vec<Note>>, impl IntoResponse> {
    let note_service = get_note_service(&headers).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    note_service
        .get_all_notes()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn get_note(headers: ApiHeaders, Path(path): Path<String>) -> Result<Json<Note>, impl IntoResponse> {
    let note_service = get_note_service(&headers).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match note_service.get_note(&path).await {
        Ok(Some(note)) => Ok(Json(note)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Note not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn create_note(headers: ApiHeaders, Json(payload): Json<CreateNote>) -> impl IntoResponse {
    let note_service = match get_note_service(&headers) {
        Ok(service) => service,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    match note_service.create_note(&payload).await {
        Ok(_) => (StatusCode::CREATED, "Note created".to_string()),
        Err(NoteServiceError::NoteAlreadyExists) => {
            (StatusCode::CONFLICT, "Note with this path already exists".to_string())
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn update_note(
    headers: ApiHeaders,
    Path(path): Path<String>,
    Json(payload): Json<UpdateNote>,
) -> impl IntoResponse {
    let note_service = match get_note_service(&headers) {
        Ok(service) => service,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    match note_service.update_note(&path, &payload).await {
        Ok(_) => (StatusCode::OK, "Note updated".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn delete_note(headers: ApiHeaders, Path(path): Path<String>) -> impl IntoResponse {
    let note_service = match get_note_service(&headers) {
        Ok(service) => service,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    match note_service.delete_note(&path).await {
        Ok(_) => (StatusCode::OK, "Note deleted".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
