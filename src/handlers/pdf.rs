use axum::{extract::{Query, State}, http::StatusCode, response::Json, Form};
use serde::Deserialize;
use crate::{models::ApiResponse, AppState};

#[derive(Debug, Deserialize)]
pub struct PdfQuery {
    pub fname: String,
}

#[derive(Debug, Deserialize)]
pub struct PdfForm {
    pub content: String,
}

pub async fn get_pdf(
    State(_state): State<AppState>,
    Query(query): Query<PdfQuery>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "fname": query.fname,
        "message": "PDF retrieved"
    })))
}

pub async fn convert_html_to_pdf(
    State(_state): State<AppState>,
    Form(form): Form<PdfForm>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "message": "PDF conversion processed",
        "content_length": form.content.len()
    })))
}