pub mod auth;
pub mod save;
pub mod run_as;
pub mod email;
pub mod user_sheet;
pub mod insert;
pub mod import;
pub mod download;
pub mod pdf;
pub mod image;
pub mod webapp;
pub mod dropbox;
pub mod inapp;
pub mod restore;
pub mod amazon;
pub mod finance;
pub mod business;

use axum::{response::Json, extract::State};
use serde_json::json;
use crate::AppState;

pub async fn home(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "message": "Aspiring Investments API",
        "version": "1.0.0",
        "status": "running"
    }))
}