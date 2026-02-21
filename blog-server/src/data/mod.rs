
pub mod user_repository_trait;
pub mod post_repository_trait;

pub mod db_user_repository;
pub mod in_memory_user_repository;

pub mod in_memory_post_repository;
pub mod db_post_repository;

pub use user_repository_trait::UserRepository;
pub use post_repository_trait::PostRepository;

