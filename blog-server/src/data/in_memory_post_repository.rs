use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::data::PostRepository;
use async_trait::async_trait;

use crate::domain::error::DomainError;
use crate::domain::post::Post;

pub struct InMemoryPostRepository {
    posts: Arc<RwLock<HashMap<i64, Post>>>,
    next_post_id: Arc<RwLock<i64>>,
}

impl InMemoryPostRepository {
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
        let timestamp = chrono::Utc::now().to_rfc3339();

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
            created_at: timestamp.to_string(),
            updated_at: timestamp.to_string(),
        };

        println!("Created post: {:?}", post);

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
        //todo!("Implement post update")
        let mut posts = self.posts.write().await;
        if let Some(post) = posts.get_mut(&id)
            && post.author_id == author_id
        {
            post.title = title;
            post.content = content;
            post.updated_at = chrono::Utc::now().to_rfc3339().to_string();
            Ok(post.clone())
        } else {
            Err(DomainError::PostNotFound)
        }
    }

    async fn delete(&self, id: i64, author_id: i64) -> Result<bool, DomainError> {
        //todo!("Implement post deletion")
        let mut posts = self.posts.write().await;
        if let Some(post) = posts.get(&id)
            && post.author_id == author_id
        {
            Ok(posts.remove(&id).is_some())
        } else {
            Err(DomainError::PostNotFound)
        }
    }

    async fn list(&self, offset: i64, limit: i64) -> Result<Vec<Post>, DomainError> {
        //todo!("Implement list posts")
        let posts = self.posts.read().await;

        println!(
            "📋 LIST - Repository pointer: {:p}, count: {}",
            self.posts,
            posts.len()
        );

        println!(
            "📋 REPO ACCESS - Type: {}, Pointer: {:p}", //, Count: {}",
            "create",                                   // or "list", "find_by_id", etc.
            self.posts.as_ref() as *const _,
            //self.posts.read().await.len()
        );

        Ok(posts
            .values()
            .cloned()
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
