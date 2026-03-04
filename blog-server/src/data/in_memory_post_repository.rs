use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::data::PostRepository;
use async_trait::async_trait;

use crate::domain::error::DomainError;
use crate::domain::post::Post;

#[allow(dead_code)]
pub struct InMemoryPostRepository {
    posts: Arc<RwLock<HashMap<i64, Post>>>,
    next_post_id: Arc<RwLock<i64>>,
}

impl InMemoryPostRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(HashMap::new())),
            next_post_id: Arc::new(RwLock::new(1)),
        }
    }
}

#[async_trait]
impl PostRepository for InMemoryPostRepository {
    async fn create(
        &self,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, DomainError> {
        let timestamp = chrono::Utc::now();

        let post_id = {
            let mut id_lock = self.next_post_id.write().await;
            let id = *id_lock;
            *id_lock += 1;
            id
        };

        let post = Post {
            id: post_id,
            title,
            content,
            author_id,
            created_at: timestamp,
            updated_at: timestamp,
        };

        //println!("Created post: {:?}", post);

        /*println!(
            "📋 REPO ACCESS - Type: {}, Pointer: {:p}, Count: {}",
            "create", // or "list", "find_by_id", etc.
            self.posts.as_ref() as *const _,
            self.posts.read().await.len()
        );*/

        self.posts.write().await.insert(post.id, post.clone());
        Ok(post)
    }

    async fn find_by_id(&self, id: i64) -> Result<Post, DomainError> {
        //todo!("Implement find by id")
        let posts = self.posts.read().await;
        posts.get(&id).cloned().ok_or(DomainError::PostNotFound)
    }

    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, DomainError> {
        let mut posts = self.posts.write().await;
        let post = posts.get_mut(&id).ok_or(DomainError::PostNotFound)?;
        if post.author_id != author_id {
            return Err(DomainError::Forbidden);
        }
        post.title = title;
        post.content = content;
        post.updated_at = chrono::Utc::now();
        Ok(post.clone())
    }

    async fn delete(&self, id: i64, author_id: i64) -> Result<bool, DomainError> {
        let mut posts = self.posts.write().await;
        let post = posts.get(&id).ok_or(DomainError::PostNotFound)?;
        if post.author_id != author_id {
            return Err(DomainError::Forbidden);
        }
        Ok(posts.remove(&id).is_some())
    }

    async fn list(&self, offset: i64, limit: i64) -> Result<Vec<Post>, DomainError> {
        let posts = self.posts.read().await;
        let mut sorted: Vec<Post> = posts.values().cloned().collect();
        sorted.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(sorted
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect())
    }

    async fn count(&self) -> Result<i64, DomainError> {
        //todo!("Implement count posts")
        let posts = self.posts.read().await;
        Ok(posts.len() as i64)
    }
}