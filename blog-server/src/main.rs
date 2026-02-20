use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_web::middleware::Logger;

use tokio::sync::RwLock;

use tracing_subscriber::EnvFilter;

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use application::blog_service::BlogService;
use data::in_memory_post_repository::InMemoryPostRepository;
use infrastructure::database::{create_pool, run_migrations};
use infrastructure::app_state::AppState;
use presentation::http_handlers::{*};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env::set_var("RUST_LOG", "debug");
    //env_logger::init();

    //tracing_subscriber::fmt::init();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    /*tracing_subscriber::fmt()
    .with_env_filter(
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug"))
    )
    .init();*/

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/blog".to_string());

    let pool = create_pool(&database_url).await.expect("Failed to create pool");
    
    run_migrations(&pool).await.expect("Failed to run migrations");

    let server_address = "0.0.0.0:8080";

    let app_state = web::Data::new(AppState {
        blog_service: RwLock::new(BlogService::new(
            std::sync::Arc::new(InMemoryPostRepository::new())
        ))
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(web::scope("/api")
                .route("/health", web::get().to(health_check))
                .route("/auth/register", web::post().to(register_user))
                .route("/auth/login", web::post().to(login_user))
                .route("/posts", web::post().to(create_post))
                .route("/posts/{id}", web::get().to(get_post))
                .route("/posts/{id}", web::put().to(update_post))
                .route("/posts/{id}", web::delete().to(delete_post))
                .route("/posts", web::get().to(get_posts))
            )
    })
    .bind(&server_address)?
    .run()
    .await
} 
