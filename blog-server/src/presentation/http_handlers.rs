use actix_web::{web, App, HttpResponse, Responder};
use serde::Deserialize;

use crate::domain::user::{RegisterUserRequest, LoginUserResponse, LoginRequest, User};
use crate::domain::post::{self, Post, CreatePostRequest, UpdatePostRequest};
use crate::infrastructure::app_state::AppState;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

pub async fn register_user(user: web::Json<RegisterUserRequest>, state: web::Data<AppState>) -> impl Responder {
    
    let register_response = LoginUserResponse {
        token: "".to_string(),
        user: User {
            id: 1,
            username: user.username.clone(),
            email: user.email.clone(),
            password_hash: "hashed_password".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        }
    };

    HttpResponse::Created().json(register_response)
}

pub async fn login_user(user: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    
    let login_response = LoginUserResponse {
        token: "fake-jwt-token".to_string(),
        user: User {
            id: 1,
            username: user.username.clone(),
            email: "user@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        }
    };

    HttpResponse::Ok().json(login_response)
}

pub async fn create_post(post: web::Json<CreatePostRequest>, state: web::Data<AppState>) -> impl Responder {
    
    let blog_service = state.blog_service.clone();//.write().await;

    let author_id = 1; //todo!("Extract auth id from token")

    let r = blog_service.create_post(post.title.clone(), post.content.clone(), author_id).await;

    match r {
        Ok(post) => HttpResponse::Created().json(post),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"error": "Failed to create post"})),
    }
}

pub async fn delete_post(path: web::Path<i64>, state: web::Data<AppState>) -> impl Responder {
    let post_id = path.into_inner();

    let auth_id = 1; //todo!("Extract auth id from token")

    let blog_service = state.blog_service.clone();//.write().await;

    let r = blog_service.delete_post(post_id, auth_id).await;

    match r {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Post deleted!"})),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({"error": "Post not found"})),
    }
}

pub async fn update_post(path: web::Path<i64>, post: web::Json<UpdatePostRequest>, state: web::Data<AppState>) -> impl Responder {
    
    let post_id = path.into_inner();

    let blog_service = state.blog_service.clone();//.write().await;

    let author_id = 1; //todo!("Extract auth id from token")

    let r = blog_service.update_post(post_id, post.title.clone(), post.content.clone(), author_id).await;

    match r {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({"error": "Post not found"})),
    }
}

pub async fn get_post(path: web::Path<i64>, state: web::Data<AppState>) -> impl Responder {
    let post_id = path.into_inner();

    if post_id <= 0 {
        return HttpResponse::NotFound().json(serde_json::json!({"error": "Post not found"}));
    }

    let blog_service = state.blog_service.clone();//.read().await;

    let post = blog_service.get_post(post_id).await;
    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({"error": "Post not found"})),
    }
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    offset: Option<u32>,
    limit: Option<u32>,
}

pub async fn get_posts(query: web::Query<PaginationQuery>, state: web::Data<AppState>) -> impl Responder {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    let blog_service = state.blog_service.clone();//.read().await;

    let posts = blog_service.get_posts(offset as i32, limit as i32).await.unwrap_or_else(|_| vec![]);
    
    HttpResponse::Ok().json(serde_json::json!({
        "posts": posts,
        "total": posts.len(),
        "offset": offset,
        "limit": limit
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health_check)));
}
