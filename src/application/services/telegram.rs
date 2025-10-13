use crate::domain::services::telegram::ITelegramService;

pub struct TelegramService;

impl TelegramService {
    pub fn new() -> TelegramService {
        TelegramService {}
    }
}

impl ITelegramService for TelegramService {
    fn verify_auth(
        &self,
        auth_data: &crate::domain::dto::telegram::TelegramAuthData,
    ) -> Result<bool, String> {
        let bot_token = std::env::var("TELEGRAM_BOT_TOKEN").map_err(|e| e.to_string())?;
        let auth_data_to_string = serde_json::to_string(auth_data).map_err(|e| e.to_string())?;
        let verification = telegram_auth_rs::validate(&auth_data_to_string, &bot_token);

        match verification {
            Ok(_) => Ok(true),
            Err(err) => match err {
                telegram_auth_rs::ValidationError::InvalidInput => Err("Invalid input".to_string()),
                telegram_auth_rs::ValidationError::InvalidHash => Err("Invalid hash".to_string()),
            },
        }
    }
}
