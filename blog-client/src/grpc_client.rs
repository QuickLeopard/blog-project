use chrono::{DateTime, Utc};
use tonic::metadata::MetadataValue;
use tonic::Request;

use crate::error::BlogClientError;
use crate::post::Post;
use crate::traits::BlogService;
use crate::user::{LoginUserResponse, User};

pub mod blog {
    tonic::include_proto!("blog");
}

fn map_grpc_status(status: tonic::Status) -> BlogClientError {
    match status.code() {
        tonic::Code::NotFound => BlogClientError::NotFound,
        tonic::Code::Unauthenticated | tonic::Code::PermissionDenied => {
            BlogClientError::Unauthorized(status.message().to_string())
        }
        tonic::Code::AlreadyExists => BlogClientError::Conflict(status.message().to_string()),
        tonic::Code::InvalidArgument => {
            BlogClientError::InvalidRequest(status.message().to_string())
        }
        _ => BlogClientError::Internal(status.message().to_string()),
    }
}

fn timestamp_to_datetime(
    ts: Option<prost_types::Timestamp>,
) -> Result<DateTime<Utc>, BlogClientError> {
    let ts = ts.ok_or_else(|| BlogClientError::Internal("Timestamp field is missing".into()))?;
    DateTime::from_timestamp(ts.seconds, ts.nanos as u32)
        .ok_or_else(|| BlogClientError::Internal("Invalid timestamp value".into()))
}

fn authed_request<T>(msg: T, token: &str) -> Result<Request<T>, BlogClientError> {
    let mut request = Request::new(msg);
    let token_value = MetadataValue::try_from(format!("Bearer {}", token))
        .map_err(|e| BlogClientError::Internal(e.to_string()))?;
    request.metadata_mut().insert("authorization", token_value);
    Ok(request)
}

fn grpc_post(post: blog::Post) -> Result<Post, BlogClientError> {
    Ok(Post {
        id: post.id,
        title: post.title,
        content: post.content,
        author_id: post.author_id,
        created_at: timestamp_to_datetime(post.created_at)?,
        updated_at: timestamp_to_datetime(post.updated_at)?,
    })
}

pub struct BlogGrpcClient {
    client: blog::blog_service_client::BlogServiceClient<tonic::transport::Channel>,
}

impl BlogGrpcClient {
    pub async fn connect(addr: String) -> Result<Self, BlogClientError> {
        let client = blog::blog_service_client::BlogServiceClient::connect(addr).await?;
        Ok(Self { client })
    }
}

#[tonic::async_trait]
impl BlogService for BlogGrpcClient {
    async fn login_user(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError> {
        let mut client = self.client.clone();
        let request = Request::new(blog::LoginRequest {
            username: username.clone(),
            password,
        });

        let response = client.login(request).await.map_err(map_grpc_status)?.into_inner();

        if !response.success {
            return Err(BlogClientError::Unauthorized(response.message));
        }

        let user = response
            .user
            .ok_or_else(|| BlogClientError::Internal("User not provided in response".into()))?;

        Ok(LoginUserResponse {
            user: User {
                id: user.id,
                username,
                email: user.email,
                created_at: timestamp_to_datetime(user.created_at)?,
            },
            token: response.token,
        })
    }

    async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError> {
        let mut client = self.client.clone();
        let request = Request::new(blog::RegisterRequest {
            username: username.clone(),
            email: email.clone(),
            password,
        });

        let response = client
            .register(request)
            .await
            .map_err(map_grpc_status)?
            .into_inner();

        if !response.success {
            return Err(BlogClientError::Conflict(response.message));
        }

        let user = response
            .user
            .ok_or_else(|| BlogClientError::Internal("User not provided in response".into()))?;

        Ok(LoginUserResponse {
            user: User {
                id: user.id,
                username,
                email,
                created_at: timestamp_to_datetime(user.created_at)?,
            },
            token: response.token,
        })
    }

    async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError> {
        let mut client = self.client.clone();
        let request = authed_request(blog::CreatePostRequest { title, content }, &token)?;

        let response = client
            .create_post(request)
            .await
            .map_err(map_grpc_status)?
            .into_inner();

        if !response.success {
            return Err(BlogClientError::Internal(response.message));
        }

        response
            .post
            .ok_or_else(|| BlogClientError::Internal("Post not provided in response".into()))
            .and_then(grpc_post)
    }

    async fn delete(&self, id: i64, token: String) -> Result<bool, BlogClientError> {
        let mut client = self.client.clone();
        let request = authed_request(blog::DeletePostRequest { id }, &token)?;

        let response = client
            .delete_post(request)
            .await
            .map_err(map_grpc_status)?
            .into_inner();

        if !response.success {
            return Err(BlogClientError::Internal(response.message));
        }

        Ok(true)
    }

    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError> {
        let mut client = self.client.clone();
        let request = authed_request(blog::UpdatePostRequest { id, title, content }, &token)?;

        let response = client
            .update_post(request)
            .await
            .map_err(map_grpc_status)?
            .into_inner();

        if !response.success {
            return Err(BlogClientError::Internal(response.message));
        }

        response
            .post
            .ok_or_else(|| BlogClientError::Internal("Post not provided in response".into()))
            .and_then(grpc_post)
    }

    async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        let mut client = self.client.clone();
        let response = client
            .get_post(Request::new(blog::GetPostRequest { id }))
            .await
            .map_err(map_grpc_status)?
            .into_inner();

        response
            .post
            .ok_or_else(|| BlogClientError::NotFound)
            .and_then(grpc_post)
    }

    async fn get_posts(&self, offset: i32, limit: i32) -> Result<Vec<Post>, BlogClientError> {
        let mut client = self.client.clone();
        let request = Request::new(blog::ListPostsRequest {
            offset: Some(offset),
            limit: Some(limit),
        });
        let response = client
            .list_posts(request)
            .await
            .map_err(map_grpc_status)?
            .into_inner();

        response
            .posts
            .into_iter()
            .map(grpc_post)
            .collect::<Result<Vec<_>, _>>()
    }
}
