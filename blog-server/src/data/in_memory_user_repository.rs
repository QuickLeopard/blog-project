
use std::collections::HashMap;
use std::sync::Arc;
use sqlx::error::DatabaseError;
use tokio::sync::RwLock;

use crate::domain::user::User;
use async_trait::async_trait;

use crate::data::UserRepository;

pub struct InMemoryUserRepository{
    users: Arc<RwLock<HashMap<i64, User>>>,
    next_user_id: Arc<RwLock<i64>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            next_user_id: Arc::new(RwLock::new(1)),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {

    async fn create(&self, username: String, email: String, password_hash: String) -> Result<User, sqlx::Error> {

        let timestamp = chrono::Utc::now().to_rfc3339();

        let user_id = {
            let mut id_lock = self.next_user_id.write().await;
            let id = *id_lock;
            *id_lock += 1;
            id
        };

        let mut repository = self.users.write().await;

        let user = User::new(user_id, username, email, password_hash, timestamp);

        repository.insert(user_id, user.clone()).ok_or(sqlx::Error::RowNotFound) //sqlx::Error::RowNotFound)

        /*if let None = repository.insert(user_id, user.clone()) {

        }

        Ok(user)*/
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        todo!("Implement find by username")
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        todo!("Implement find by id")
    }

}
