use crate::{models::ApiResponse, AppState};
use axum::{
    extract::{Extension, State},
    response::Json,
};
use uuid::Uuid;

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
