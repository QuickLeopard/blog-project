use async_trait::async_trait;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::error::DomainError;
use crate::domain::user::User;

use crate::data::UserRepository;

#[allow(dead_code)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<i64, User>>>,
    next_user_id: Arc<RwLock<i64>>,
}

impl InMemoryUserRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            next_user_id: Arc::new(RwLock::new(1)),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<User, DomainError> {
        let mut repository = self.users.write().await;

        if repository.values().any(|u| u.username == username)
            || repository.values().any(|u| u.email == email)
        {
            return Err(DomainError::UserAlreadyExists(
                "User with this username or email already exists".to_string(),
            ));
        }

        let timestamp = chrono::Utc::now();

        let user_id = {
            let mut id_lock = self.next_user_id.write().await;
            let id = *id_lock;
            *id_lock += 1;
            id
        };

        let user = User::new(user_id, username, email, password_hash, timestamp);

        repository.insert(user_id, user.clone());
        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let repository = self.users.read().await;
        Ok(repository
            .values()
            .find(|u| u.username == username)
            .cloned())
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError> {
        let repository = self.users.read().await;
        Ok(repository.get(&id).cloned())
    }
}
