use axum::{extract::State, Json};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    auth::{telegram::verify_telegram_auth, AuthResponse, TelegramAuthData},
    models::{CreateUserRequest, User},
    utils::{ApiError, ApiResult},
    AppState,
};

/// Authenticate with Telegram
#[utoipa::path(
    post,
    path = "/auth/telegram",
    request_body = TelegramAuthData,
    responses(
        (status = 200, description = "Authentication successful", body = AuthResponse),
        (status = 401, description = "Authentication failed")
    ),
    tag = "Authentication"
)]
pub async fn telegram_auth(
    State(state): State<AppState>,
    Json(auth_data): Json<TelegramAuthData>,
) -> ApiResult<Json<AuthResponse>> {
    // Verify Telegram authentication
    let is_valid = verify_telegram_auth(&auth_data, &state.config.telegram_bot_token)?;

    if !is_valid {
        return Err(ApiError::Auth(
            "Invalid Telegram authentication".to_string(),
        ));
    }

    // Check if user exists
    let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE telegram_id = $1")
        .bind(auth_data.id)
        .fetch_optional(&state.db)
        .await?;

    let user = match existing_user {
        Some(user) => {
            // Update user's last activity
            sqlx::query("UPDATE users SET updated_at = $1 WHERE id = $2")
                .bind(Utc::now())
                .bind(user.id)
                .execute(&state.db)
                .await?;
            user
        }
        None => {
            // Create new user
            let create_request = CreateUserRequest {
                telegram_id: auth_data.id,
                username: auth_data.username,
                first_name: auth_data.first_name,
                last_name: auth_data.last_name,
            };

            let user_id = Uuid::new_v4();
            let now = Utc::now();

            sqlx::query_as::<_, User>(
                r#"
                INSERT INTO users (id, telegram_id, username, first_name, last_name, is_active, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING *
                "#
            )
            .bind(user_id)
            .bind(create_request.telegram_id)
            .bind(create_request.username)
            .bind(create_request.first_name)
            .bind(create_request.last_name)
            .bind(true)
            .bind(now)
            .bind(now)
            .fetch_one(&state.db)
            .await?
        }
    };

    // Generate JWT token
    let (token, expires_at) = state
        .jwt_service
        .generate_token(user.id, user.telegram_id)?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
        expires_at,
    }))
}
