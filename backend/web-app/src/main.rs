use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod assets;

use api::{create_note, delete_note, get_note, list_notes, update_note};
use assets::static_handler;

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
