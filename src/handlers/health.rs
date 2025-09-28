use axum::{extract::State, Json};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{utils::ApiResult, AppState};

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Serialize, ToSchema)]
pub struct DatabaseHealthResponse {
    pub database: String,
    pub redis: String,
}

/// Get API health status
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse)
    ),
    tag = "Health"
)]
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Get detailed health status including database connections
#[utoipa::path(
    get,
    path = "/health/detailed",
    responses(
        (status = 200, description = "Detailed health check", body = DatabaseHealthResponse)
    ),
    tag = "Health"
)]
pub async fn detailed_health_check(
    State(state): State<AppState>,
) -> ApiResult<Json<DatabaseHealthResponse>> {
    // Check database connection
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("error: {}", e),
    };

    // Check Redis connection
    let redis_status = match state.redis.get_connection() {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("error: {}", e),
    };

    Ok(Json(DatabaseHealthResponse {
        database: db_status,
        redis: redis_status,
    }))
}
