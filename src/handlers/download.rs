use axum::{extract::State, http::StatusCode, response::Json, Form};
use serde::Deserialize;
use crate::{models::ApiResponse, AppState};

#[derive(Debug, Deserialize)]
pub struct DownloadForm {
    pub r#type: String,
    pub content: String,
}

pub async fn download_file(
    State(_state): State<AppState>,
    Form(form): Form<DownloadForm>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "type": form.r#type,
        "message": "Download processed"
    })))
}