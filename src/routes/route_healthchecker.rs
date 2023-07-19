use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

/// Axum Route Handler to check server status.
async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str =
        "Building a simple CRUD API with Rust, Supabase Client (Postgrest), and Axum";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub fn healthchecker_router() -> Router {
    Router::new().route("/api/healthchecker", get(health_checker_handler))
}
