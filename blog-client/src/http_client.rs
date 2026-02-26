use async_trait::async_trait;

use reqwest;

use crate::post::{ListPostsResponse, Post};
use crate::traits::BlogService;

pub struct HttpClient {
    url: String,
}

impl HttpClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[async_trait]
impl BlogService for HttpClient {
    async fn get_post(&self, id: i64) -> anyhow::Result<Post> {
        let response = reqwest::get(format!("{}/api/posts/{}", self.url, id))
            .await?
            .json::<Post>()
            .await?;

        Ok(response)
    }

    async fn get_posts(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Post>> {
        let response = reqwest::get(format!(
            "{}/api/posts?offset={}&limit={}",
            self.url, offset, limit
        ))
        .await?
        .json::<ListPostsResponse>()
        .await?;

        Ok(response.posts)

        //Ok(vec![]) // Placeholder
    }

    async fn count_posts(&self) -> anyhow::Result<i32> {
        todo!("Implement HTTP client to count posts from the server")
        //Ok(0) // Placeholder
    }
}
