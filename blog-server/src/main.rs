use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_web::middleware::Logger;

use tokio::sync::RwLock;
use std::sync::Arc;

use tonic::transport::Server;

use tracing_subscriber::EnvFilter;
use tracing::info;

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

use presentation::grpc_service::{BlogServiceImpl, ServerState};
use presentation::grpc_service::blog::blog_service_server::BlogServiceServer;

pub mod blog {
    tonic::include_proto!("blog");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("blog_descriptor");
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
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

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/blog".to_string());

    let pool = create_pool(&database_url).await?;
    
    run_migrations(&pool).await?;

    let http_address = "0.0.0.0:8080";
    let grpc_address = "127.0.0.1:50051".parse()?;

    let app_state = web::Data::new(AppState {
        blog_service: Arc::new(
            RwLock::new(
                BlogService::new(
                    Arc::new(InMemoryPostRepository::new())
                )
            )
        )

    });

    let service = BlogServiceImpl::new(app_state.blog_service.clone());
    
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(blog::FILE_DESCRIPTOR_SET)
        .build()?;

    let http_server = HttpServer::new(move || {
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
    .bind(&http_address)?
    .run();

    info!("🚀 Blog HTTP server started on {}", http_address);
    info!("🚀 Blog gRPC server starting on {}", grpc_address);
    
    let grpc_server = Server::builder()
        .add_service(BlogServiceServer::new(service))
        .add_service(reflection_service)
        .serve(grpc_address);

    tokio::select! {
        res = http_server => {
            res?;
        }
        res = grpc_server => {
            res?;
        }
    }

    Ok(())
} 
