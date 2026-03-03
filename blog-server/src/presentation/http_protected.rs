use actix_web::{HttpResponse, Responder, delete, post, put, web};
use tracing::info;

use crate::domain::error::DomainError;
use crate::domain::post::{CreatePostRequest, UpdatePostRequest};
use crate::infrastructure::app_state::AppState;
use crate::presentation::middleware::AuthenticatedUser;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_post)
        .service(delete_post)
        .service(update_post);
}

#[post("/posts")]
pub async fn create_post(
    post: web::Json<CreatePostRequest>,
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
) -> Result<impl Responder, DomainError> {
    let blog_service = state.blog_service.clone(); //.write().await;

    info!(user_id = auth_user.user_id, title = %post.title, "Creating post");

    let created_post = blog_service
        .create_post(post.title.clone(), post.content.clone(), auth_user.user_id)
        .await?;

    Ok(HttpResponse::Created().json(created_post))
}

#[delete("/posts/{id}")]
pub async fn delete_post(
    path: web::Path<i64>,
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
) -> Result<impl Responder, DomainError> {
    let post_id = path.into_inner();

    if post_id <= 0 {
        return Err(DomainError::PostNotFound);
    }

    let blog_service = state.blog_service.clone(); //.write().await;

    blog_service.delete_post(post_id, auth_user.user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Post deleted!"})))
}

#[put("/posts/{id}")]
pub async fn update_post(
    path: web::Path<i64>,
    post: web::Json<UpdatePostRequest>,
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
) -> Result<impl Responder, DomainError> {
    let post_id = path.into_inner();

    if post_id <= 0 {
        return Err(DomainError::PostNotFound);
    }

    let blog_service = state.blog_service.clone(); //.write().await;

    let updated_post = blog_service
        .update_post(
            post_id,
            post.title.clone(),
            post.content.clone(),
            auth_user.user_id,
        )
        .await?;

    Ok(HttpResponse::Ok().json(updated_post))
}
