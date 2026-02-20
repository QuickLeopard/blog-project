use crate::application::blog_service::BlogService;
use tokio::sync::RwLock;

pub struct AppState {
    pub blog_service: RwLock<BlogService>,
}