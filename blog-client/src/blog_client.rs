use crate::error::BlogClientError;
use crate::post::Post;
use crate::traits::BlogService;
use crate::user::LoginUserResponse;

pub struct BlogClient {
    transport: Box<dyn BlogService>,
}

impl BlogClient {
    pub fn new(transport: Box<dyn BlogService>) -> Self {
        Self { transport }
    }

    pub async fn login_user(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError> {
        self.transport.login_user(username, password).await
    }

    pub async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError> {
        self.transport
            .register_user(username, email, password)
            .await
    }

    pub async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError> {
        self.transport.create_post(title, content, token).await
    }

    pub async fn delete_post(
        &self,
        id: i64,
        token: String,
    ) -> Result<bool, BlogClientError> {
        self.transport.delete(id, token).await
    }

    pub async fn update_post(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError> {
        self.transport.update(id, title, content, token).await
    }

    pub async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        self.transport.get_post(id).await
    }

    pub async fn get_posts(
        &self,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<Post>, BlogClientError> {
        self.transport.get_posts(offset, limit).await
    }
}
