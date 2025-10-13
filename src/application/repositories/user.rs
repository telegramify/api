use crate::domain::{
    entities::user::User,
    repositories::user::{
        CreateUserDto,
        IUserRepository,
    },
};

pub struct MockUserRepository {
    users: Vec<User>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        MockUserRepository { users: vec![] }
    }
}

impl IUserRepository for MockUserRepository {
    fn create_user(&mut self, dto: &CreateUserDto) -> Result<i64, String> {
        let user = User {
            id: 1, // In a real implementation, this would be generated
            username: dto.username.clone(),
            email: dto.email.clone(),
            first_name: dto.first_name.clone(),
            last_name: dto.last_name.clone(),
            password: dto.password.clone(),
            telegram_id: dto.telegram_id,
            photo_url: None,
        };
        self.users.push(user);
        Ok(1)
    }
}
