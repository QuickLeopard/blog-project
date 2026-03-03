//use crate::blog::blog_service_client::Client;
use chrono::{DateTime, Utc};
use tonic::Request;

use tonic::metadata::MetadataValue;

use anyhow::Context;

use crate::post::Post;
use crate::traits::BlogService;
use crate::user::{LoginUserResponse, User};

pub mod blog {
    tonic::include_proto!("blog");
}

// Convert Option<Timestamp> to DateTime<Utc>
fn timestamp_to_datetime(ts: Option<prost_types::Timestamp>) -> anyhow::Result<DateTime<Utc>> {
    let ts = ts.context("Timestamp field is missing")?;
    DateTime::from_timestamp(ts.seconds, ts.nanos as u32).context("Invalid timestamp value")
}

pub struct BlogGrpcClient {
    client: blog::blog_service_client::BlogServiceClient<tonic::transport::Channel>,
}

impl BlogGrpcClient {
    pub async fn connect(addr: String) -> anyhow::Result<Self> {
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
    ) -> anyhow::Result<LoginUserResponse> {
        let mut client = self.client.clone();
        let request = Request::new(blog::LoginRequest {
            username: username.clone(),
            password,
        });

        let response = client.login(request).await?.into_inner();

        if !response.success {
            return Err(anyhow::anyhow!("{}", response.message));
        }

        let user = response
            .user
            .ok_or_else(|| anyhow::anyhow!("User not provided in response"))?;

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
    ) -> anyhow::Result<LoginUserResponse> {
        let mut client = self.client.clone();
        let request = Request::new(blog::RegisterRequest {
            username: username.clone(),
            email: email.clone(),
            password: password.clone(),
        });

        let response = client.register(request).await?.into_inner();

        if !response.success {
            return Err(anyhow::anyhow!("{}", response.message));
        }

        let user = response
            .user
            .ok_or_else(|| anyhow::anyhow!("User not provided in response"))?;

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
    ) -> anyhow::Result<Post> {
        let mut client = self.client.clone();
        let mut request = Request::new(blog::CreatePostRequest { title, content });

        // Add token to metadata
        let token_value = MetadataValue::try_from(format!("Bearer {}", token))?;
        request.metadata_mut().insert("authorization", token_value);

        let response = client.create_post(request).await?.into_inner();

        if !response.success {
            return Err(anyhow::anyhow!("{}", response.message));
        }

        let post = response
            .post
            .ok_or_else(|| anyhow::anyhow!("Post not provided in response"))?;

        Ok(Post {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            created_at: timestamp_to_datetime(post.created_at)?,
            updated_at: timestamp_to_datetime(post.updated_at)?,
        })
    }

    async fn delete(&self, id: i64, token: String) -> anyhow::Result<bool> {
        let mut client = self.client.clone();
        let mut request = Request::new(blog::DeletePostRequest { id });

        // Add token to metadata
        let token_value = MetadataValue::try_from(format!("Bearer {}", token))?;
        request.metadata_mut().insert("authorization", token_value);

        let response = client.delete_post(request).await?.into_inner();

        if !response.success {
            return Err(anyhow::anyhow!("{}", response.message));
        }

        Ok(true)
    }

    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> anyhow::Result<Post> {
        let mut client = self.client.clone();
        let mut request = Request::new(blog::UpdatePostRequest { id, title, content });

        // Add token to metadata
        let token_value = MetadataValue::try_from(format!("Bearer {}", token))?;
        request.metadata_mut().insert("authorization", token_value);

        let response = client.update_post(request).await?.into_inner();

        if !response.success {
            return Err(anyhow::anyhow!("{}", response.message));
        }

        let post = response
            .post
            .ok_or_else(|| anyhow::anyhow!("Post not provided in response"))?;

        Ok(Post {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            created_at: timestamp_to_datetime(post.created_at)?,
            updated_at: timestamp_to_datetime(post.updated_at)?,
        })
    }

    async fn get_post(&self, id: i64) -> anyhow::Result<Post> {
        let mut client = self.client.clone();
        client
            .get_post(Request::new(blog::GetPostRequest { id }))
            .await?
            .into_inner()
            .post
            .ok_or_else(|| anyhow::anyhow!("Post not found"))
            .and_then(|post| {
                Ok(Post {
                    id: post.id,
                    title: post.title,
                    content: post.content,
                    author_id: post.author_id,
                    created_at: timestamp_to_datetime(post.created_at)?,
                    updated_at: timestamp_to_datetime(post.updated_at)?,
                })
            })
    }

    async fn get_posts(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Post>> {
        let mut client = self.client.clone();
        let request = Request::new(blog::ListPostsRequest {
            offset: Some(offset),
            limit: Some(limit),
        });
        let response = client.list_posts(request).await?.into_inner();

        // Map to Result<Post>, then collect into Result<Vec<Post>>
        response
            .posts
            .into_iter()
            .map(|post| {
                Ok(Post {
                    id: post.id,
                    title: post.title,
                    content: post.content,
                    author_id: post.author_id,
                    created_at: timestamp_to_datetime(post.created_at)?,
                    updated_at: timestamp_to_datetime(post.updated_at)?,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()
    }

    /*async fn count_posts(&self) -> anyhow::Result<i64> {
        let mut client = self.client.clone();
        let request = Request::new(blog::CountPostsRequest {});
        let response = client.count_posts(request).await?.into_inner();
        Ok(response.count)
    }*/
}
