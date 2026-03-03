use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use tonic::{Request, Response, Status};

use std::sync::Arc;

use tracing::debug;

use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;

use crate::domain::post::Post;

use crate::blog;

// Convert DateTime<Utc> to Timestamp
fn datetime_to_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}

#[derive(Clone)]
pub struct BlogGrpcService {
    auth_service: Arc<AuthService>,
    blog_service: Arc<BlogService>,
}

impl BlogGrpcService {
    pub fn new(auth_service: Arc<AuthService>, blog_service: Arc<BlogService>) -> Self {
        // ✅ No RwLock in parameter
        Self {
            auth_service,
            blog_service,
        }
    }

    fn post_to_grpc(post: &Post) -> blog::Post {
        blog::Post {
            id: post.id,
            title: post.title.clone(),
            content: post.content.clone(),
            author_id: post.author_id,
            author_username: String::new(), // TODO: Populate from User Service
            created_at: Some(datetime_to_timestamp(post.created_at)),
            updated_at: Some(datetime_to_timestamp(post.updated_at)),
        }
    }

    #[allow(clippy::result_large_err)]
    fn get_auth_token<T>(request: &Request<T>) -> Result<String, Status> {
        request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.strip_prefix("Bearer ").unwrap_or(s).trim().to_string())
            .ok_or_else(|| Status::unauthenticated("Invalid or missing auth token"))
    }
}

#[tonic::async_trait]
impl blog::blog_service_server::BlogService for BlogGrpcService {
    async fn health_check(
        &self,
        _request: Request<blog::HealthCheckRequest>,
    ) -> Result<Response<blog::HealthCheckResponse>, Status> {
        let response = blog::HealthCheckResponse {
            status: "OK".to_string(),
        };
        Ok(Response::new(response))
    }

    async fn register(
        &self,
        request: Request<blog::RegisterRequest>,
    ) -> Result<Response<blog::AuthResponse>, Status> {
        let request = request.into_inner();

        let (user, token) = self
            .auth_service
            .register(
                request.username.clone(),
                request.email.clone(),
                request.password.clone(),
            )
            .await?;

        let response = blog::AuthResponse {
            success: true,
            message: "User registered".to_string(),
            token,
            user: Some(blog::User {
                id: user.id,
                username: user.username.clone(),
                email: user.email.clone(),
                created_at: Some(datetime_to_timestamp(user.created_at)),
            }),
        };

        Ok(Response::new(response))
    }

    async fn login(
        &self,
        request: Request<blog::LoginRequest>,
    ) -> Result<Response<blog::AuthResponse>, Status> {
        let request = request.into_inner();

        let (user, token) = self
            .auth_service
            .login(request.username.clone(), request.password.clone())
            .await?;

        let response = blog::AuthResponse {
            success: true,
            message: "User logged in".to_string(),
            token,
            user: Some(blog::User {
                id: user.id,
                username: user.username.clone(),
                email: user.email.clone(),
                created_at: Some(datetime_to_timestamp(user.created_at)),
            }),
        };

        Ok(Response::new(response))
    }

    async fn create_post(
        &self,
        request: Request<blog::CreatePostRequest>,
    ) -> Result<Response<blog::PostResponse>, Status> {
        //let token = request.metadata().get("authorization");

        let auth_token = Self::get_auth_token(&request)?;

        let claims = self.auth_service.verify_token(&auth_token)?;

        let request = request.into_inner();

        let post = self
            .blog_service
            .create_post(request.title, request.content, claims.user_id)
            .await?;

        let grpc_post = Self::post_to_grpc(&post);

        let response = blog::PostResponse {
            success: true,
            message: "Post created".to_string(),
            post: Some(grpc_post),
        };

        Ok(Response::new(response))
    }

    async fn update_post(
        &self,
        request: Request<blog::UpdatePostRequest>,
    ) -> Result<Response<blog::PostResponse>, Status> {
        let auth_token = Self::get_auth_token(&request)?;

        let claims = self.auth_service.verify_token(&auth_token)?;

        let request = request.into_inner();

        let post = self
            .blog_service
            .update_post(request.id, request.title, request.content, claims.user_id)
            .await?;

        let grpc_post = Self::post_to_grpc(&post);

        let response = blog::PostResponse {
            success: true,
            message: "Post updated".to_string(),
            post: Some(grpc_post),
        };

        Ok(Response::new(response))
    }

    async fn delete_post(
        &self,
        request: Request<blog::DeletePostRequest>,
    ) -> Result<Response<blog::DeletePostResponse>, Status> {
        let auth_token = Self::get_auth_token(&request)?;

        let claims = self.auth_service.verify_token(&auth_token)?;

        let request = request.into_inner();

        let r = self
            .blog_service
            .delete_post(request.id, claims.user_id)
            .await?;

        let response = blog::DeletePostResponse {
            success: r,
            message: if r {
                "Post deleted".to_string()
            } else {
                "Failed to delete post".to_string()
            },
        };

        Ok(Response::new(response))
    }

    async fn get_post(
        &self,
        request: Request<blog::GetPostRequest>,
    ) -> Result<Response<blog::PostResponse>, Status> {
        let request = request.into_inner();

        let post = self.blog_service.get_post(request.id).await?;

        let grpc_post = Self::post_to_grpc(&post);

        let response = blog::PostResponse {
            success: true,
            message: "Post found".to_string(),
            post: Some(grpc_post),
        };

        Ok(Response::new(response))
    }

    async fn list_posts(
        &self,
        request: Request<blog::ListPostsRequest>,
    ) -> Result<Response<blog::ListPostsResponse>, Status> {
        //todo!("Implement delete_post")
        let request = request.into_inner();

        let offset = request.offset.unwrap_or(0);
        let limit = request.limit.unwrap_or(10);

        let posts = self
            .blog_service
            .get_posts(offset, limit)
            .await
            .unwrap_or_else(|_| vec![]);

        debug!("📋 gRPC LIST - count: {}", posts.len());

        //let posts = self.state.blog_service.read().await.get_posts(request.offset, request.limit).await.unwrap_or_else(|_| vec![]);

        debug!("📋 gRPC LIST - offset: {}, limit: {}", offset, limit);

        //println!("📋 LIST - Repository pointer: {:p}, count: {}", posts, posts.len());

        let response = blog::ListPostsResponse {
            posts: posts.iter().map(Self::post_to_grpc).collect(),
            total: posts.len() as i32,
            offset,
            limit,
        };

        Ok(Response::new(response))
    }
}
