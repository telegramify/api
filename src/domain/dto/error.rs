use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct BaseApiError {
    error: String,
}

impl BaseApiError {
    pub fn new(error: String) -> Self {
        Self { error }
    }

    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub enum ApiError {
    Auth(String),

    Validation(String),

    BadRequest(String),

    NotFound(String),

    Interal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            ApiError::Auth(ref err) => (StatusCode::UNAUTHORIZED, err.as_str()),
            ApiError::BadRequest(ref err) => (StatusCode::BAD_REQUEST, err.as_str()),
            ApiError::Validation(ref err) => (StatusCode::BAD_REQUEST, err.as_str()),
            ApiError::NotFound(ref err) => (StatusCode::NOT_FOUND, err.as_str()),
            ApiError::Interal(ref err) => (StatusCode::INTERNAL_SERVER_ERROR, err.as_str()),
        };
        let body = BaseApiError::new(err_msg.to_owned()).to_json();

        (status, body).into_response()
    }
}
