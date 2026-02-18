use tonic::{Request, Response, Status};

pub mod blog {
    tonic::include_proto!("blog");
}

pub struct BlogServiceImpl;

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
        todo!("Implement list_posts")
    }
}
