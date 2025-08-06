use crate::{models::ApiResponse, AppState};
use axum::{
    extract::{Query, State},
    response::Json,
    Form,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ImageQuery {
    pub fname: String,
}

#[derive(Debug, Deserialize)]
pub struct ImageForm {
    pub content: String,
    pub suffix: String,
}

pub async fn get_image(
    State(_state): State<AppState>,
    Query(query): Query<ImageQuery>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "fname": query.fname,
        "message": "Image retrieved"
    })))
}

pub async fn upload_image(
    State(_state): State<AppState>,
    Form(form): Form<ImageForm>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "message": "Image uploaded",
        "suffix": form.suffix
    })))
}
