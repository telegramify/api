pub struct CreateUserDto {
    pub username: String,
    pub email: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub password: Option<String>,
    pub telegram_id: i64,
}

pub trait IUserRepository {
    fn create_user(&mut self, user: &CreateUserDto) -> Result<i64, String>;
}
