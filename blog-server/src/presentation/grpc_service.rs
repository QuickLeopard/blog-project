use tonic::{Request, Response, Status};

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;

use crate::domain::post::Post;

pub mod blog {
    tonic::include_proto!("blog");
}

/*#[derive(Clone)]
pub struct ServerState {
    // Here you can add shared state, e.g., database connection pool
    pub blog_service: Arc<RwLock<BlogService>>,
}

impl ServerState {
    pub fn new(blog_service: Arc<RwLock<BlogService>>) -> Self {
        Self { blog_service }
    }
}

#[derive(Clone)]
pub struct BlogServiceImpl {
    state: ServerState
}

impl BlogServiceImpl {
    pub fn new(service: Arc<RwLock<BlogService>>) -> Self {
        Self {
            state: ServerState::new(service)
        }
    }
}*/

#[derive(Clone)]
pub struct BlogGrpcService {
    auth_service: Arc<AuthService>,
    blog_service: Arc<BlogService>, // ✅ Just Arc, no RwLock
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
            created_at: post.created_at.clone(),
            updated_at: post.updated_at.clone(),
        }
    }

    fn get_auth_token(request: &Request<()>) -> Result<String, Status> {
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
        request: Request<blog::HealthCheckRequest>,
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
        todo!("Implement register")
    }

    async fn login(
        &self,
        request: Request<blog::LoginRequest>,
    ) -> Result<Response<blog::AuthResponse>, Status> {
        todo!("Implement login")
    }

    async fn create_post(
        &self,
        request: Request<blog::CreatePostRequest>,
    ) -> Result<Response<blog::PostResponse>, Status> {
        let request = request.into_inner();

        let auth_id = 1;

        let post = self
            .blog_service
            .create_post(request.title, request.content, auth_id)
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
        let request = request.into_inner();

        let auth_id = 1;

        let post = self
            .blog_service
            .update_post(request.id, request.title, request.content, auth_id)
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
        //todo!("Implement delete_post")

        let request = request.into_inner();

        let auth_id = 1;

        let r = self.blog_service.delete_post(request.id, auth_id).await?;

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
        //todo!("Implement get_post")

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

        println!("📋 gRPC LIST - count: {}", posts.len());

        //let posts = self.state.blog_service.read().await.get_posts(request.offset, request.limit).await.unwrap_or_else(|_| vec![]);

        println!("📋 gRPC LIST - offset: {}, limit: {}", offset, limit);

        //println!("📋 LIST - Repository pointer: {:p}, count: {}", posts, posts.len());

        let response = blog::ListPostsResponse {
            posts: posts.iter().map(|p| Self::post_to_grpc(p)).collect(),
            total: posts.len() as i32,
        };

        Ok(Response::new(response))
    }
}
