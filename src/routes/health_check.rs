use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[axum::debug_handler]
pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "online"
        })),
    )
}
