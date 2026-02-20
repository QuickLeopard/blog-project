use crate::application::blog_service::BlogService;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct AppState {
    pub blog_service: Arc<RwLock<BlogService>>,
}