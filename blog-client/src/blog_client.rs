use crate::http_client::HttpClient;
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
