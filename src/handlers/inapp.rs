use crate::{models::ApiResponse, AppState};
use axum::{extract::State, response::Json, Form};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InAppForm {
    pub app: String,
    pub user: String,
}

pub async fn handle_inapp(
    State(_state): State<AppState>,
    Form(form): Form<InAppForm>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "app": form.app,
        "user": form.user,
        "message": "In-app purchase processed"
    })))
}
