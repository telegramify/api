use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::dto::error::ApiError;

pub type ApiResult<T> = Result<Json<BaseResponseDto<T>>, ApiError>;

#[derive(PartialEq, Debug, ToSchema, Clone, Serialize, Deserialize)]
pub struct BaseResponseDto<T: Default> {
    pub data: T,
    pub message: Option<String>,
    pub code: u16,
}

impl<T: Default> Default for BaseResponseDto<T> {
    fn default() -> Self {
        BaseResponseDto {
            data: T::default(),
            message: None,
            code: 200,
        }
    }
}

impl<T: Default> BaseResponseDto<T> {
    pub fn builder() -> BaseResponseDto<T> {
        BaseResponseDto::<T>::default()
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = data;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    pub fn to_json(self) -> Json<Self> {
        Json(Self {
            code: self.code,
            data: self.data,
            message: self.message,
        })
    }
}
