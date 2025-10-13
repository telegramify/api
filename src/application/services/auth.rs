use crate::domain::repositories::user::IUserRepository;

pub struct AuthService<'a> {
    repository: &'a mut dyn IUserRepository,
}

impl<'a> AuthService<'a> {
    pub fn new(repository: &'a mut dyn IUserRepository) -> Self {
        AuthService { repository }
    }

    pub fn login_via_telegram(
        &mut self,
        dto: &crate::domain::repositories::user::CreateUserDto,
    ) -> Result<i64, String> {
        self.repository.create_user(dto)
    }
}
