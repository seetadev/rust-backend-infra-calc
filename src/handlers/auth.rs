use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    Form,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    auth::{create_jwt, generate_random_string, hash_password, verify_password},
    models::ApiResponse,
    services::email::EmailService,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LostPasswordForm {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordResetForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordResetQuery {
    pub u: String,
    pub d: String,
}

pub async fn login_page(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "page": "login",
        "message": "Please provide email and password"
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let user = match state.db.get_user_by_email(&form.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(Json(ApiResponse::error("Invalid credentials".to_string()))),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if !verify_password(&form.password, &user.password_hash) {
        return Ok(Json(ApiResponse::error("Invalid credentials".to_string())));
    }

    let token = match create_jwt(user.id, &state.config.jwt_secret) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(ApiResponse::success(json!({
        "token": token,
        "user": {
            "id": user.id,
            "email": user.email
        }
    }))))
}

pub async fn logout(State(_state): State<AppState>) -> Json<ApiResponse<()>> {
    Json(ApiResponse::success_simple())
}

pub async fn register_page(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "page": "register",
        "message": "Please provide email and password to register"
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Check if user already exists
    match state.db.get_user_by_email(&form.email).await {
        Ok(Some(_)) => return Ok(Json(ApiResponse::error("User already exists".to_string()))),
        Ok(None) => {}
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    // Hash password
    let password_hash = match hash_password(&form.password) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Create user
    let user = match state.db.create_user(&form.email, &password_hash).await {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Create JWT token
    let token = match create_jwt(user.id, &state.config.jwt_secret) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(ApiResponse::success(json!({
        "token": token,
        "user": {
            "id": user.id,
            "email": user.email
        }
    }))))
}

pub async fn lost_password_page(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "page": "lost_password",
        "message": "Please provide your email to reset password"
    }))
}

pub async fn lost_password(
    State(state): State<AppState>,
    Form(form): Form<LostPasswordForm>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user = match state.db.get_user_by_email(&form.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(Json(ApiResponse::error("User not found".to_string()))),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let dongle = generate_random_string(20);

    if let Err(_) = state.db.set_user_dongle(user.id, &dongle).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let reset_link = format!(
        "http://localhost:8080/pwreset?u={}&d={}",
        form.email, dongle
    );
    let message = format!(
        "Please click the following link to reset password for user {}\n{}",
        form.email, reset_link
    );

    let email_service = EmailService::new(&state.config);
    if let Err(_) = email_service
        .await
        .send_email(&form.email, "Password Reset", &message)
        .await
    {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(ApiResponse::success_simple()))
}

pub async fn password_reset_page(
    State(state): State<AppState>,
    Query(query): Query<PasswordResetQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user = match state.db.get_user_by_email(&query.u).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok(Json(json!({
                "page": "password_reset_invalid",
                "message": "Invalid reset link"
            })))
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if user.dongle.as_ref() != Some(&query.d) {
        return Ok(Json(json!({
            "page": "password_reset_invalid",
            "message": "Invalid reset link"
        })));
    }

    Ok(Json(json!({
        "page": "password_reset",
        "email": query.u,
        "message": "Please enter your new password"
    })))
}

pub async fn password_reset(
    State(state): State<AppState>,
    Form(form): Form<PasswordResetForm>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user = match state.db.get_user_by_email(&form.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(Json(ApiResponse::error("User not found".to_string()))),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let password_hash = match hash_password(&form.password) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if let Err(_) = state.db.update_user_password(user.id, &password_hash).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Clear the dongle
    if let Err(_) = state.db.set_user_dongle(user.id, "").await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(ApiResponse::success_simple()))
}
