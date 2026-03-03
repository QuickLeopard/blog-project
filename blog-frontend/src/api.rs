use gloo_net::http::Request;

use crate::types::*;

const API_BASE: &str = "";

fn to_err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

async fn http_error(resp: gloo_net::http::Response) -> String {
    resp.json::<ErrorResponse>()
        .await
        .map(|e| e.error)
        .unwrap_or_else(|_| format!("HTTP {}", resp.status()))
}

pub async fn login(username: &str, password: &str) -> Result<LoginUserResponse, String> {
    let resp = Request::post(&format!("{}/api/auth/login", API_BASE))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginRequest {
                username: username.to_string(),
                password: password.to_string(),
            })
            .map_err(to_err)?,
        )
        .map_err(to_err)?
        .send()
        .await
        .map_err(to_err)?;

    if !resp.ok() {
        return Err(http_error(resp).await);
    }

    resp.json::<LoginUserResponse>().await.map_err(to_err)
}

// POST /api/auth/register — body: { username, email, password } → LoginUserResponse
pub async fn register(
    username: &str,
    email: &str,
    password: &str,
) -> Result<LoginUserResponse, String> {
    let resp = Request::post(&format!("{}/api/auth/register", API_BASE))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&RegisterUserRequest {
                username: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
            })
            .map_err(to_err)?,
        )
        .map_err(|e: gloo_net::Error| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(http_error(resp).await);
    }

    resp.json::<LoginUserResponse>()
        .await
        .map_err(|e| e.to_string())
}

// GET /api/posts?offset={offset}&limit={limit} → ListPostsResponse
pub async fn get_posts(offset: i32, limit: i32) -> Result<ListPostsResponse, String> {
    let resp = Request::get(&format!(
        "{}/api/posts?offset={}&limit={}",
        API_BASE, offset, limit
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(http_error(resp).await);
    }

    resp.json::<ListPostsResponse>()
        .await
        .map_err(|e| e.to_string())
}

// GET /api/posts/{id} → Post
pub async fn get_post(id: i64) -> Result<Post, String> {
    let resp = Request::get(&format!("{}/api/posts/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(http_error(resp).await);
    }

    resp.json::<Post>().await.map_err(|e| e.to_string())
}

// POST /api/posts — header: Authorization: Bearer {token}, body: { title, content } → Post
pub async fn create_post(title: &str, content: &str, token: &str) -> Result<Post, String> {
    let resp = Request::post(&format!("{}/api/posts", API_BASE))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            serde_json::to_string(&PostRequest {
                title: title.to_string(),
                content: content.to_string(),
            })
            .map_err(to_err)?,
        )
        .map_err(|e: gloo_net::Error| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(http_error(resp).await);
    }

    resp.json::<Post>().await.map_err(|e| e.to_string())
}

// PUT /api/posts/{id} — header: Authorization: Bearer {token}, body: { title, content } → Post
pub async fn update_post(id: i64, title: &str, content: &str, token: &str) -> Result<Post, String> {
    let resp = Request::put(&format!("{}/api/posts/{}", API_BASE, id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(
            serde_json::to_string(&PostRequest {
                title: title.to_string(),
                content: content.to_string(),
            })
            .map_err(to_err)?,
        )
        .map_err(|e: gloo_net::Error| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(http_error(resp).await);
    }

    resp.json::<Post>().await.map_err(|e| e.to_string())
}

// DELETE /api/posts/{id} — header: Authorization: Bearer {token} → DeletePostResponse
pub async fn delete_post(id: i64, token: &str) -> Result<DeletePostResponse, String> {
    let resp = Request::delete(&format!("{}/api/posts/{}", API_BASE, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(http_error(resp).await);
    }

    resp.json::<DeletePostResponse>()
        .await
        .map_err(|e| e.to_string())
}
