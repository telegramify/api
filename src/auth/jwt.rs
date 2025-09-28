use crate::utils::ApiResult;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub telegram_id: i64,
    pub exp: i64, // Expiration time
    pub iat: i64, // Issued at
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_token(
        &self,
        user_id: Uuid,
        telegram_id: i64,
    ) -> ApiResult<(String, chrono::DateTime<Utc>)> {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours

        let claims = Claims {
            sub: user_id.to_string(),
            telegram_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        Ok((token, exp))
    }

    pub fn verify_token(&self, token: &str) -> ApiResult<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())?;

        Ok(token_data.claims)
    }
}
