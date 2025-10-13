use crate::{
    application::services::telegram::TelegramService,
    domain::{
        dto::{
            common::{ApiResult, BaseResponseDto},
            error::{ApiError, BaseApiError},
            telegram::TelegramAuthData,
        },
        services::telegram::ITelegramService,
    },
};
use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, Default)]
struct TelegramLoginResponse {
    token: String,
}

#[derive(Deserialize, Serialize)]
struct CredentialsLoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct CredentialsLoginResponse {
    token: String,
}

pub fn create_router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(telegram_login))
}

#[
    utoipa::path(
        post,
        path = "/telegram",
        tag = "auth",
        responses(
            (status = StatusCode::OK, description = "Login via Telegram", body=BaseResponseDto<TelegramLoginResponse>),
            (status = StatusCode::UNAUTHORIZED, description = "Unauthorized error", body=BaseApiError)
        ),
        request_body = TelegramAuthData
    )
]
async fn telegram_login(payload: Json<TelegramAuthData>) -> ApiResult<TelegramLoginResponse> {
    let telegram_auth_service = TelegramService::new();
    let telegram_auth_response = telegram_auth_service.verify_auth(&payload);
    match telegram_auth_response {
        Err(auth_error) => Err(ApiError::Auth(auth_error)),
        _ => {
            let dto = BaseResponseDto::<TelegramLoginResponse>::builder()
                .data(TelegramLoginResponse {
                    token: "some-token".to_owned(),
                })
                .code(StatusCode::OK.as_u16())
                .message("Successfully authenticated!".to_owned());
            Ok(dto.to_json())
        }
    }
}
