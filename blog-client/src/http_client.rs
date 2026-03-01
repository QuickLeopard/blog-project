use async_trait::async_trait;

use reqwest;

use crate::post::{CreatePostRequest, ListPostsResponse, Post, UpdatePostRequest};
use crate::traits::BlogService;
use crate::user::{LoginUserResponse, RegisterUserRequest};

pub struct HttpClient {
    url: String,
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl BlogService for HttpClient {
    async fn login_user(
        &self,
        username: String,
        password: String,
    ) -> anyhow::Result<LoginUserResponse> {
        let login_request = crate::user::LoginRequest { username, password };

        let response = self
            .client
            .post(format!("{}/api/auth/login", self.url))
            .json(&login_request)
            .send()
            .await?
            .error_for_status()?
            .json::<LoginUserResponse>()
            .await?;

        Ok(response)
    }
    async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> anyhow::Result<LoginUserResponse> {
        let register_request = RegisterUserRequest {
            username,
            email,
            password,
        };

        let response = self
            .client
            .post(format!("{}/api/auth/register", self.url))
            .json(&register_request)
            .send()
            .await?
            .error_for_status()?
            .json::<LoginUserResponse>()
            .await?;

        Ok(response)
    }

    async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> anyhow::Result<Post> {
        let post = CreatePostRequest { title, content };

        let response = self
            .client
            .post(format!("{}/api/posts", self.url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&post)
            .send()
            .await?
            .error_for_status()?
            .json::<Post>()
            .await?;

        Ok(response)
    }

    async fn delete(&self, id: i64, token: String) -> anyhow::Result<bool> {
        let response = self
            .client
            .delete(format!("{}/api/posts/{}", self.url, id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?
            .error_for_status()?;
        Ok(response.status().as_u16() == 200)
    }

    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> anyhow::Result<Post> {
        let post = UpdatePostRequest { title, content };

        let response = self
            .client
            .put(format!("{}/api/posts/{}", self.url, id))
            .header("Authorization", format!("Bearer {}", token))
            .json(&post)
            .send()
            .await?
            .error_for_status()?
            .json::<Post>()
            .await?;

        Ok(response)
    }

    async fn get_post(&self, id: i64) -> anyhow::Result<Post> {
        let response = self
            .client
            .get(format!("{}/api/posts/{}", self.url, id))
            .send()
            .await?
            .error_for_status()?
            .json::<Post>()
            .await?;

        Ok(response)
    }

    async fn get_posts(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Post>> {
        let response = self
            .client
            .get(format!(
                "{}/api/posts?offset={}&limit={}",
                self.url, offset, limit
            ))
            .send()
            .await?
            .error_for_status()?
            .json::<ListPostsResponse>()
            .await?;

        Ok(response.posts)

        //Ok(vec![]) // Placeholder
    }

    /*async fn count_posts(&self) -> anyhow::Result<i64> {
        todo!("Implement HTTP client to count posts from the server")
        //Ok(0) // Placeholder
    }*/
}
