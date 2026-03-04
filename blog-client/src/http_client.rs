use async_trait::async_trait;

use crate::error::BlogClientError;

/// Timeout for all outgoing HTTP requests. Applies to the entire request
/// lifecycle (connection + send + receive).
const HTTP_TIMEOUT_SECS: u64 = 30;
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
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(HTTP_TIMEOUT_SECS))
                .build()
                .unwrap_or_default(),
        }
    }
}

async fn map_http_error(resp: reqwest::Response) -> BlogClientError {
    let status = resp.status().as_u16();
    let body = resp.text().await.unwrap_or_default();

    let msg = serde_json::from_str::<serde_json::Value>(&body)
        .ok()
        .and_then(|v| v.get("error").and_then(|e| e.as_str()).map(String::from))
        .unwrap_or(body);

    match status {
        401 | 403 => BlogClientError::Unauthorized(msg),
        404 => BlogClientError::NotFound,
        409 => BlogClientError::Conflict(msg),
        400 | 422 => BlogClientError::InvalidRequest(msg),
        _ => BlogClientError::Internal(format!("HTTP {status}: {msg}")),
    }
}

#[async_trait]
impl BlogService for HttpClient {
    async fn login_user(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError> {
        let login_request = crate::user::LoginRequest { username, password };

        let resp = self
            .client
            .post(format!("{}/api/auth/login", self.url))
            .json(&login_request)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(map_http_error(resp).await);
        }

        Ok(resp.json::<LoginUserResponse>().await?)
    }

    async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError> {
        let register_request = RegisterUserRequest {
            username,
            email,
            password,
        };

        let resp = self
            .client
            .post(format!("{}/api/auth/register", self.url))
            .json(&register_request)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(map_http_error(resp).await);
        }

        Ok(resp.json::<LoginUserResponse>().await?)
    }

    async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError> {
        let post = CreatePostRequest { title, content };

        let resp = self
            .client
            .post(format!("{}/api/posts", self.url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&post)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(map_http_error(resp).await);
        }

        Ok(resp.json::<Post>().await?)
    }

    async fn delete(&self, id: i64, token: String) -> Result<bool, BlogClientError> {
        let resp = self
            .client
            .delete(format!("{}/api/posts/{}", self.url, id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(map_http_error(resp).await);
        }

        Ok(true)
    }

    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError> {
        let post = UpdatePostRequest { title, content };

        let resp = self
            .client
            .put(format!("{}/api/posts/{}", self.url, id))
            .header("Authorization", format!("Bearer {}", token))
            .json(&post)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(map_http_error(resp).await);
        }

        Ok(resp.json::<Post>().await?)
    }

    async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        let resp = self
            .client
            .get(format!("{}/api/posts/{}", self.url, id))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(map_http_error(resp).await);
        }

        Ok(resp.json::<Post>().await?)
    }

    async fn get_posts(&self, offset: i32, limit: i32) -> Result<Vec<Post>, BlogClientError> {
        let resp = self
            .client
            .get(format!(
                "{}/api/posts?offset={}&limit={}",
                self.url, offset, limit
            ))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(map_http_error(resp).await);
        }

        let list = resp.json::<ListPostsResponse>().await?;
        Ok(list.posts)
    }
}
