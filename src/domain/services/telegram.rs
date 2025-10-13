use crate::domain::dto::telegram::TelegramAuthData;

pub trait ITelegramService {
    fn verify_auth(&self, auth_data: &TelegramAuthData) -> Result<bool, String>;
}
