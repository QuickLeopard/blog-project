//use crate::blog::blog_service_client::Client;
use crate::blog_client::*;
use tonic::Request;

use crate::post::{ListPostsResponse, Post};
use crate::traits::BlogService;

pub mod blog {
    tonic::include_proto!("blog");
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
    async fn get_post(&self, id: i64) -> anyhow::Result<Post> {
        let mut client = self.client.clone();
        client
            .get_post(Request::new(blog::GetPostRequest { id }))
            .await?
            .into_inner()
            .post
            .ok_or_else(|| anyhow::anyhow!("Post not found"))
            .map(|post| Post {
                id: post.id,
                title: post.title,
                content: post.content,
                author_id: post.author_id,
                created_at: post.created_at,
                updated_at: post.updated_at,
            })
    }

    async fn get_posts(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Post>> {
        let mut client = self.client.clone();
        let request = Request::new(blog::ListPostsRequest {
            offset: Some(offset),
            limit: Some(limit),
        });
        let response = client.list_posts(request).await?.into_inner();
        Ok(response
            .posts
            .into_iter()
            .map(|post| Post {
                id: post.id,
                title: post.title,
                content: post.content,
                author_id: post.author_id,
                created_at: post.created_at,
                updated_at: post.updated_at,
            })
            .collect())
    }

    async fn count_posts(&self) -> anyhow::Result<i32> {
        todo!("Implement gRPC client to count posts from the server")
    }
}
