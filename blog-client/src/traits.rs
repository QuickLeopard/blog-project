use async_trait::async_trait;

use crate::post::Post;
use crate::user::LoginUserResponse;

#[async_trait]
pub trait BlogService: Send + Sync {
    /*async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        author_id: i64,
    ) -> anyhow::Result<()>;*/
    async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> anyhow::Result<Post>;
    async fn delete(&self, id: i64, token: String) -> anyhow::Result<bool>;
    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> anyhow::Result<Post>;
    async fn login_user(
        &self,
        username: String,
        password: String,
    ) -> anyhow::Result<LoginUserResponse>;
    async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> anyhow::Result<LoginUserResponse>;
    async fn get_post(&self, id: i64) -> anyhow::Result<Post>;
    async fn get_posts(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Post>>;
    //async fn count_posts(&self) -> anyhow::Result<i64>;
}
