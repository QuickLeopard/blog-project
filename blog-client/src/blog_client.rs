
use crate::post::Post;
use crate::http_client::HttpClient;

pub struct BlogClient {
    //todo!("Implement BlogClient to interact with the HTTP client and provide a higher-level API for the application")
    transport: HttpClient,
}

impl BlogClient {
    pub fn new(transport: HttpClient) -> Self {
        Self { transport }
    }

    pub async fn get_post(&self, id: i64) -> anyhow::Result<Post> {
        self.transport.get_post(id).await
    }

    pub async fn get_posts(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Post>> {
        self.transport.get_posts(offset, limit)
    }
}