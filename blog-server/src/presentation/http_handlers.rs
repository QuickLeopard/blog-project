use actix_web::{HttpResponse, Responder, Scope, delete, get, post, put, web};
use serde::{Deserialize, de};

use crate::domain::error::DomainError;
use crate::domain::post::{self, CreatePostRequest, Post, UpdatePostRequest};
use crate::domain::user::{LoginRequest, LoginUserResponse, RegisterUserRequest, User};
use crate::infrastructure::app_state::AppState;
use crate::infrastructure::hash::{hash_password, verify_password};

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[post("/auth/register")]
pub async fn register_user(
    user: web::Json<RegisterUserRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, DomainError> {
    let auth_service = state.auth_service.clone();
    let user = auth_service
        .register(
            user.username.clone(),
            user.email.clone(),
            user.password.clone(),
        )
        .await?;

    let register_response = LoginUserResponse {
        token: "fake-jwt-token".to_string(),
        user: user,
    };

    Ok(HttpResponse::Created().json(register_response))
}

#[post("/auth/login")]
pub async fn login_user(
    user: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, DomainError> {
    let auth_service = state.auth_service.clone();

    let user = auth_service
        .login(user.username.clone(), user.password.clone())
        .await?;

    let login_response = LoginUserResponse {
        token: "fake-jwt-token".to_string(),
        user: user,
    };

    Ok(HttpResponse::Ok().json(login_response))
}

#[post("/posts")]
pub async fn create_post(
    post: web::Json<CreatePostRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let blog_service = state.blog_service.clone(); //.write().await;

    let author_id = 1; //todo!("Extract auth id from token")

    let r = blog_service
        .create_post(post.title.clone(), post.content.clone(), author_id)
        .await;

    match r {
        Ok(post) => HttpResponse::Created().json(post),
        Err(_) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": "Failed to create post"})),
    }
}

#[delete("/posts/{id}")]
pub async fn delete_post(path: web::Path<i64>, state: web::Data<AppState>) -> impl Responder {
    let post_id = path.into_inner();

    let auth_id = 1; //todo!("Extract auth id from token")

    let blog_service = state.blog_service.clone(); //.write().await;

    let r = blog_service.delete_post(post_id, auth_id).await;

    match r {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Post deleted!"})),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({"error": "Post not found"})),
    }
}

#[put("/posts/{id}")]
pub async fn update_post(
    path: web::Path<i64>,
    post: web::Json<UpdatePostRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let post_id = path.into_inner();

    let blog_service = state.blog_service.clone(); //.write().await;

    let author_id = 1; //todo!("Extract auth id from token")

    let r = blog_service
        .update_post(post_id, post.title.clone(), post.content.clone(), author_id)
        .await;

    match r {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({"error": "Post not found"})),
    }
}

#[get("/posts/{id}")]
pub async fn get_post(
    path: web::Path<i64>,
    state: web::Data<AppState>,
) -> Result<impl Responder, DomainError> {
    //impl Responder {
    let post_id = path.into_inner();

    if post_id <= 0 {
        return Err(DomainError::PostNotFound);
    }

    let blog_service = state.blog_service.clone(); //.read().await;

    let post = blog_service.get_post(post_id).await?;

    Ok(HttpResponse::Ok().json(post))

    //let post = blog_service.get_post(post_id).await?;

    /*match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({"error": "Post not found"})),
    }*/

    //HttpResponse::Ok().json(post)
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    offset: Option<u32>,
    limit: Option<u32>,
}

#[get("/posts")]
pub async fn get_posts(
    query: web::Query<PaginationQuery>,
    state: web::Data<AppState>,
) -> Result<impl Responder, DomainError> {
    //impl Responder {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    let blog_service = state.blog_service.clone(); //.read().await;

    let posts = blog_service.get_posts(offset as i32, limit as i32).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
    "posts": posts,
    "total": posts.len(),
    "offset": offset,
    "limit": limit
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let scope: Scope = web::scope("/api")
        .service(health_check)
        .service(register_user)
        .service(login_user)
        .service(create_post)
        .service(get_post)
        .service(update_post)
        .service(delete_post)
        .service(get_posts);

    cfg.service(scope);
}
