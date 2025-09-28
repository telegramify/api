use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    models::{User, UserResponse},
    utils::{ApiError, ApiResult},
    AppState,
};

/// Get user by ID
#[utoipa::path(
    get,
    path = "/users/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Users"
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<UserResponse>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}

/// Get all users
#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = Vec<UserResponse>)
    ),
    tag = "Users"
)]
pub async fn list_users(State(state): State<AppState>) -> ApiResult<Json<Vec<UserResponse>>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await?;

    let user_responses: Vec<UserResponse> = users.into_iter().map(|user| user.into()).collect();

    Ok(Json(user_responses))
}
