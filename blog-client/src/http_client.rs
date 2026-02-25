
use reqwest;

use crate::post::Post;

pub struct HttpClient {
    url: String,
}

impl HttpClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn get_post(&self, id: i64) -> anyhow::Result<Post> {

        let response = reqwest::get(format!("{}/posts/{}", self.url, id))
        .await?
        .json::<Post>()
        .await?;

        Ok(response)
    }

    pub fn get_posts(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Post>> {
        //todo!("Implement HTTP client to fetch posts from the server")



        Ok(vec![]) // Placeholder
    }

}   