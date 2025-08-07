use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
    Form,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{ApiResponse, FileListEntry},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct SaveForm {
    pub fname: String,
    pub data: String,
}

pub async fn list_files(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<ApiResponse<Vec<FileListEntry>>>, StatusCode> {
    let files = match state.db.list_files(user_id, "home/").await {
        Ok(files) => files,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let entries: Vec<FileListEntry> = files
        .into_iter()
        .map(|file| FileListEntry {
            fname: file.path.split('/').last().unwrap_or("").to_string(),
            created_at: file.created_at,
            updated_at: file.updated_at,
        })
        .collect();

    Ok(Json(ApiResponse::success(entries)))
}

pub async fn save_file(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Form(form): Form<SaveForm>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let file_path = format!("home/{}", form.fname);

    // Check if file exists
    match state.db.get_file(user_id, &file_path).await {
        Ok(Some(_)) => {
            // Update existing file
            if let Err(_) = state.db.update_file(user_id, &file_path, &form.data).await {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        Ok(None) => {
            // Create new file
            if let Err(_) = state.db.create_file(user_id, &file_path, &form.data).await {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    Ok(Json(ApiResponse::success("Done".to_string())))
}
