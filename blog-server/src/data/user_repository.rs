use crate::domain::user::User;

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn create(&self, username: String, email: String, password_hash: String) -> Result<User, sqlx::Error> {
        todo!("Implement user creation")
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        todo!("Implement find by username")
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        todo!("Implement find by id")
    }
}
