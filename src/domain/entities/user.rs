pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub photo_url: Option<String>,
    pub email: Option<String>,
    pub telegram_id: i64,
    pub password: Option<String>,
}
