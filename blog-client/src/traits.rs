use async_trait::async_trait;

use crate::error::BlogClientError;
use crate::post::Post;
use crate::user::LoginUserResponse;

#[async_trait]
pub trait BlogService: Send + Sync {
    async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError>;
    async fn delete(&self, id: i64, token: String) -> Result<bool, BlogClientError>;
    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError>;
    async fn login_user(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError>;
    async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError>;
    async fn get_post(&self, id: i64) -> Result<Post, BlogClientError>;
    async fn get_posts(&self, offset: i32, limit: i32) -> Result<Vec<Post>, BlogClientError>;
}
