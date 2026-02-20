use tonic::{Request, Response, Status};

use tokio::sync::RwLock;
use std::sync::Arc;

use crate::application::blog_service::BlogService;

pub mod blog {
    tonic::include_proto!("blog");
}

#[derive(Clone)]
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
}

#[tonic::async_trait]
impl blog::blog_service_server::BlogService for BlogServiceImpl {
    async fn register(&self, request: Request<blog::RegisterRequest>) -> Result<Response<blog::AuthResponse>, Status> {
        todo!("Implement register")
    }

    async fn login(&self, request: Request<blog::LoginRequest>) -> Result<Response<blog::AuthResponse>, Status> {
        todo!("Implement login")
    }

    async fn create_post(&self, request: Request<blog::CreatePostRequest>) -> Result<Response<blog::PostResponse>, Status> {
        todo!("Implement create_post")
    }

    async fn get_post(&self, request: Request<blog::GetPostRequest>) -> Result<Response<blog::PostResponse>, Status> {
        todo!("Implement get_post")
    }

    async fn update_post(&self, request: Request<blog::UpdatePostRequest>) -> Result<Response<blog::PostResponse>, Status> {
        todo!("Implement update_post")
    }

    async fn delete_post(&self, request: Request<blog::DeletePostRequest>) -> Result<Response<blog::DeletePostResponse>, Status> {
        todo!("Implement delete_post")
    }

    async fn list_posts(&self, request: Request<blog::ListPostsRequest>) -> Result<Response<blog::ListPostsResponse>, Status> {
        //todo!("Implement delete_post")
        let request = request.into_inner();

        let posts = self.state.blog_service.read().await.get_posts(request.offset, request.limit).await.unwrap_or_else(|_| vec![]);
    
        let response = blog::ListPostsResponse {
            posts: posts.iter().map(|p| blog::Post {
                id: p.id,
                title: p.title.clone(),
                content: p.content.clone(),
                author_id: p.author_id,
                author_username: "author_username".to_string(), //todo!("Fetch author username"),
                created_at: p.created_at.clone(),
                updated_at: p.updated_at.clone(),
            }
            ).collect(),
            total: posts.len() as i32,
        };

        Ok(Response::new(response)) 
    }
}
