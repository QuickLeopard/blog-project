use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::domain::user::{RegisterUserRequest, LoginUserResponse, LoginRequest, User};
use crate::domain::post::{self, Post};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

pub async fn register_user(user: web::Json<RegisterUserRequest>) -> impl Responder {
    
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

pub async fn login_user(user: web::Json<LoginRequest>) -> impl Responder {
    
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

pub async fn create_post(post: web::Json<Post>) -> impl Responder {
    
    HttpResponse::Created().json(post)
}

pub async fn delete_post() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"message": "Post deleted!"}))
}

pub async fn update_post(post: web::Json<Post>) -> impl Responder {
    
    HttpResponse::Ok().json(post)
}

pub async fn get_post(path: web::Path<i64>) -> impl Responder {
    let post_id = path.into_inner();

    if post_id <= 0 {
        return HttpResponse::NotFound().json(serde_json::json!({"error": "Post not found"}));
    }

    let post = Post {
        id: post_id as i64,
        title: format!("Post {}", post_id),
        content: "This is a sample post.".to_string(),
        author_id: 1,
        created_at: "2024-01-01T00:00:00Z".to_string(),
        updated_at: "2024-01-01T00:00:00Z".to_string(),
    };
    
    HttpResponse::Ok().json(post)
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    offset: Option<u32>,
    limit: Option<u32>,
}

pub async fn get_posts(query: web::Query<PaginationQuery>) -> impl Responder {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    let posts: Vec<Post> = vec![];
    
    HttpResponse::Ok().json(format!(
        "{{ posts: {:?}, total: {}, offset: {}, limit: {} }}", posts, posts.len(), offset, limit
    ))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health_check)));
}
