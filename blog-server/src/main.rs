use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Result, web};

use std::sync::Arc;
use tokio::sync::RwLock;

use tonic::transport::Server;

//use tracing_subscriber::EnvFilter;
use tracing::info;

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use application::auth_service::AuthService;
use application::blog_service::BlogService;

use data::in_memory_post_repository::InMemoryPostRepository;
use data::in_memory_user_repository::InMemoryUserRepository;

use infrastructure::app_state::AppState;
use infrastructure::database::{create_pool, run_migrations};
use presentation::http_handlers::*;

use presentation::grpc_service::BlogGrpcService; //, ServerState};
use presentation::grpc_service::blog::blog_service_server::BlogServiceServer;

use crate::infrastructure::jwt::JwtService;

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

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/blog".to_string());

    let pool = create_pool(&database_url).await?;

    run_migrations(&pool).await?;

    let http_address = "0.0.0.0:3000";
    let grpc_address = "0.0.0.0:50051".parse()?;

    let secret_token = std::env::var("SECRET_TOKEN").unwrap_or_else(|_| "wt35y4urtjfgjhfgjfjfgjgfjfgjrtj454e5634tafazf".to_string());

    let auth_service = Arc::new(
        //RwLock::new(
        AuthService::new(Arc::new(InMemoryUserRepository::new()), JwtService::new(&secret_token)), //)
    );

    let blog_service = Arc::new(
        //RwLock::new(
        BlogService::new(Arc::new(InMemoryPostRepository::new())), //)
    );

    let app_state = web::Data::new(AppState {
        blog_service: blog_service.clone(),
        auth_service: auth_service.clone(),
    });

    let service = BlogGrpcService::new(auth_service.clone(), blog_service.clone());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(blog::FILE_DESCRIPTOR_SET)
        .build()?;

    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .configure(configure)
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
        result = http_server => {
            result?;
        }
        result = grpc_server => {
            result?;
        }
    }

    Ok(())
}
