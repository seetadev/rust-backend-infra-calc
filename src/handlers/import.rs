use axum::{extract::{Extension, State}, http::StatusCode, response::Json};
use uuid::Uuid;
use crate::{models::ApiResponse, AppState};

pub async fn import_page(
    State(_state): State<AppState>,
    Extension(_user_id): Extension<Uuid>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "page": "import",
        "message": "Import functionality"
    })))
}

pub async fn handle_import(
    State(_state): State<AppState>,
    Extension(_user_id): Extension<Uuid>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "message": "Import handled"
    })))
}