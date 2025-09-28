use axum::{
    extract::{Multipart, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    utils::{ApiError, ApiResult},
    AppState,
};

#[derive(Serialize, ToSchema)]
pub struct FileUploadResponse {
    pub file_id: String,
    pub filename: String,
    pub size: u64,
    pub content_type: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FormDataRequest {
    pub name: String,
    pub email: Option<String>,
    pub message: String,
}

/// Upload a file
#[utoipa::path(
    post,
    path = "/upload",
    request_body(content = String, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "File uploaded successfully", body = FileUploadResponse),
        (status = 400, description = "Bad request"),
        (status = 413, description = "File too large")
    ),
    tag = "Files"
)]
pub async fn upload_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> ApiResult<Json<FileUploadResponse>> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(format!("Invalid multipart data: {}", e)))?
    {
        let name = field.name().unwrap_or("unknown");

        if name == "file" {
            let filename = field.file_name().unwrap_or("unknown").to_string();

            let content_type = field.content_type().map(|ct| ct.to_string());
            let data = field
                .bytes()
                .await
                .map_err(|e| ApiError::BadRequest(format!("Failed to read file data: {}", e)))?;

            // Check file size
            if data.len() > state.config.max_file_size {
                return Err(ApiError::BadRequest(
                    "File size exceeds maximum allowed size".to_string(),
                ));
            }

            // Generate unique file ID and path
            let file_id = Uuid::new_v4().to_string();
            let file_extension = PathBuf::from(&filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_string();

            let stored_filename = if file_extension.is_empty() {
                file_id.clone()
            } else {
                format!("{}.{}", file_id, file_extension)
            };

            let file_path = PathBuf::from(&state.config.upload_dir).join(&stored_filename);

            // Save file
            let mut file = fs::File::create(&file_path).await?;
            file.write_all(&data).await?;
            file.flush().await?;

            return Ok(Json(FileUploadResponse {
                file_id,
                filename,
                size: data.len() as u64,
                content_type,
                url: format!("/files/{}", stored_filename),
            }));
        }
    }

    Err(ApiError::BadRequest("No file field found".to_string()))
}

/// Process form data
#[utoipa::path(
    post,
    path = "/form",
    request_body = FormDataRequest,
    responses(
        (status = 200, description = "Form processed successfully"),
        (status = 400, description = "Bad request")
    ),
    tag = "Forms"
)]
pub async fn process_form(
    Json(form_data): Json<FormDataRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    // Basic validation
    if form_data.name.trim().is_empty() {
        return Err(ApiError::Validation("Name is required".to_string()));
    }

    if form_data.message.trim().is_empty() {
        return Err(ApiError::Validation("Message is required".to_string()));
    }

    // Process the form data (in a real application, you might save it to database)
    tracing::info!("Processing form data for: {}", form_data.name);

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Form data processed successfully",
        "data": {
            "name": form_data.name,
            "email": form_data.email,
            "message_length": form_data.message.len()
        }
    })))
}
