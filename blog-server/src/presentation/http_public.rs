use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;

/// Default number of posts returned per page when `?limit` is not specified.
const DEFAULT_PAGE_LIMIT: u32 = 10;

/// Default starting offset when `?offset` is not specified.
const DEFAULT_PAGE_OFFSET: u32 = 0;

use crate::domain::error::DomainError;
use crate::domain::post::ListPostsResponse;
use crate::domain::user::{LoginRequest, LoginUserResponse, RegisterUserRequest};
use crate::infrastructure::app_state::AppState;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(register_user)
        .service(login_user)
        .service(get_post)
        .service(get_posts);
}

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
    let (user, token) = auth_service
        .register(
            user.username.clone(),
            user.email.clone(),
            user.password.clone(),
        )
        .await?;

    let register_response = LoginUserResponse { token, user };

    Ok(HttpResponse::Created().json(register_response))
}

#[post("/auth/login")]
pub async fn login_user(
    user: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, DomainError> {
    let auth_service = state.auth_service.clone();

    let (user, token) = auth_service
        .login(user.username.clone(), user.password.clone())
        .await?;

    let login_response = LoginUserResponse { token, user };

    Ok(HttpResponse::Ok().json(login_response))
}

#[get("/posts/{id}")]
pub async fn get_post(
    path: web::Path<i64>,
    state: web::Data<AppState>,
) -> Result<impl Responder, DomainError> {
    let post_id = path.into_inner();

    if post_id <= 0 {
        return Err(DomainError::PostNotFound);
    }

    let blog_service = state.blog_service.clone();

    let post = blog_service.get_post(post_id).await?;

    Ok(HttpResponse::Ok().json(post))
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
    let offset = query.offset.unwrap_or(DEFAULT_PAGE_OFFSET);
    let limit = query.limit.unwrap_or(DEFAULT_PAGE_LIMIT);

    let blog_service = state.blog_service.clone();

    let posts = blog_service.get_posts(offset as i32, limit as i32).await?;
    let total = blog_service.count_posts().await? as i32;

    let list_posts_response = ListPostsResponse {
        posts,
        total,
        offset: offset as i32,
        limit: limit as i32,
    };

    Ok(HttpResponse::Ok().json(list_posts_response))
}
