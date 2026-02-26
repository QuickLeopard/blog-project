
use crate::post::Post;
use crate::traits::BlogService;

pub struct BlogClient {
    //todo!("Implement BlogClient to interact with the HTTP client and provide a higher-level API for the application")
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
    ) -> anyhow::Result<crate::user::LoginUserResponse> {
        self.transport.login_user(username, password).await
    }

    pub async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> anyhow::Result<crate::user::LoginUserResponse> {
        self.transport
            .register_user(username, email, password)
            .await
    }

    pub async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> anyhow::Result<Post> {
        self.transport.create_post(title, content, token).await
    }

    pub async fn delete_post(&self, id: i64, token: String) -> anyhow::Result<bool> {
        self.transport.delete(id, token).await
    }

    pub async fn update_post(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> anyhow::Result<Post> {
        self.transport.update(id, title, content, token).await
    }

    pub async fn get_post(&self, id: i64) -> anyhow::Result<Post> {
        self.transport.get_post(id).await
    }

    pub async fn get_posts(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Post>> {
        self.transport.get_posts(offset, limit).await
    }

    pub async fn count_posts(&self) -> anyhow::Result<i32> {
        self.transport.count_posts().await
    }
}
