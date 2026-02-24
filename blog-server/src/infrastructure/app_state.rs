use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub blog_service: Arc<BlogService>, //Arc<RwLock<BlogService>>,
}
