use axum::{extract::State, response::Json, Form};
use serde::Deserialize;
use crate::{models::ApiResponse, AppState};

#[derive(Debug, Deserialize)]
pub struct DropboxForm {
    pub action: String,
}

pub async fn handle_dropbox(
    State(_state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "message": "Dropbox functionality"
    })))
}

pub async fn handle_dropbox_post(
    State(_state): State<AppState>,
    Form(form): Form<DropboxForm>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "action": form.action,
        "message": "Dropbox action processed"
    })))
}