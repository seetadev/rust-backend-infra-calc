use axum::{extract::{Extension, State}, http::StatusCode, response::Json, Form};
use serde::Deserialize;
use uuid::Uuid;
use crate::{models::ApiResponse, services::email::EmailService, AppState};

#[derive(Debug, Deserialize)]
pub struct EmailForm {
    pub to: String,
    pub subject: String,
    pub text: String,
    pub data: String,
    pub appname: String,
}

pub async fn send_email(
    State(state): State<AppState>,
    Extension(_user_id): Extension<Uuid>,
    Form(form): Form<EmailForm>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let email_service = EmailService::new(&state.config);
    
    let html_content = format!(
        "<div><p>{}</p></div>{}",
        form.text,
        form.data
    );
    
    let subject = if form.subject.is_empty() {
        format!("Shared {}", form.appname)
    } else {
        form.subject
    };
    
    match email_service.send_html_email(&form.to, &subject, &html_content).await {
        Ok(_) => Ok(Json(ApiResponse::success(form.to))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}