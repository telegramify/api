use crate::utils::{ApiError, ApiResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct TelegramAuthData {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo_url: Option<String>,
    pub auth_date: i64,
    pub hash: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: uuid::Uuid,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

pub fn verify_telegram_auth(auth_data: &TelegramAuthData, bot_token: &str) -> ApiResult<bool> {
    let mut data_check = HashMap::new();
    data_check.insert("id", auth_data.id.to_string());
    data_check.insert("first_name", auth_data.first_name.clone());

    if let Some(ref last_name) = auth_data.last_name {
        data_check.insert("last_name", last_name.clone());
    }

    if let Some(ref username) = auth_data.username {
        data_check.insert("username", username.clone());
    }

    if let Some(ref photo_url) = auth_data.photo_url {
        data_check.insert("photo_url", photo_url.clone());
    }

    data_check.insert("auth_date", auth_data.auth_date.to_string());

    // Sort keys and create check string
    let mut keys: Vec<_> = data_check.keys().collect();
    keys.sort();

    let check_string = keys
        .iter()
        .map(|key| format!("{}={}", key, data_check[*key]))
        .collect::<Vec<_>>()
        .join("\n");

    // Create secret key from bot token
    let secret_key = Sha256::digest(bot_token.as_bytes());

    // Create hash from check string
    let mut mac = hmac::Hmac::<Sha256>::new_from_slice(&secret_key)
        .map_err(|e| ApiError::Auth(format!("Failed to create HMAC: {}", e)))?;

    use hmac::Mac;
    mac.update(check_string.as_bytes());
    let result = mac.finalize();
    let calculated_hash = hex::encode(result.into_bytes());

    Ok(calculated_hash == auth_data.hash)
}

// Re-export for convenience
pub use hmac;
