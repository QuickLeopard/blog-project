use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};

use actix_cors::Cors;
use actix_web::dev::AppConfig;

use std::sync::Arc;

use tonic::transport::Server;

use tracing::info;
use tracing_subscriber::EnvFilter;

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use application::auth_service::AuthService;
use application::blog_service::BlogService;

use infrastructure::app_state::AppState;
use infrastructure::database::{create_pool, run_migrations};

use presentation::http_protected;
use presentation::http_public;
//use presentation::middleware::jwt_validator;

use presentation::grpc_service::BlogGrpcService; //, ServerState};

use crate::data::db_post_repository::DBPostRepository;
use crate::data::db_user_repository::DbUserRepository;
use crate::infrastructure::jwt::JwtService;

pub mod blog {
    tonic::include_proto!("blog");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("blog_descriptor");
}

use blog::blog_service_server::BlogServiceServer;

/// Port the Actix-Web HTTP REST API binds to.
const HTTP_PORT: u16 = 3000;

/// Port the Tonic gRPC server binds to.
const GRPC_PORT: u16 = 50051;

/// CORS preflight cache duration in seconds.
const CORS_MAX_AGE_SECS: usize = 3600;

fn build_cors(_config: &AppConfig) -> Cors {
    let cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_any_header()
        .max_age(CORS_MAX_AGE_SECS);

    match std::env::var("CORS_ORIGIN") {
        Ok(origins) => origins
            .split(',')
            .fold(cors, |c, origin| c.allowed_origin(origin.trim())),
        Err(_) => cors.allow_any_origin(),
    }
}

//#[actix_web::main]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/blog".to_string());

    let pool = create_pool(&database_url).await?;

    run_migrations(&pool).await?;

    let http_address = format!("0.0.0.0:{HTTP_PORT}");
    let grpc_address = format!("0.0.0.0:{GRPC_PORT}").parse()?;

    let secret_token = std::env::var("SECRET_TOKEN")
        .map_err(|_| anyhow::anyhow!("SECRET_TOKEN environment variable must be set"))?;

    //let auth_middleware = HttpAuthentication::bearer(jwt_validator);

    let auth_service = Arc::new(
        //RwLock::new(
        AuthService::new(
            //Arc::new(InMemoryUserRepository::new()),
            Arc::new(DbUserRepository::new(pool.clone())),
            JwtService::new(&secret_token),
        ), //)
    );

    let blog_service = Arc::new(
        //RwLock::new(
        BlogService::new(
            Arc::new(DBPostRepository::new(pool.clone())),
            //Arc::new(InMemoryPostRepository::new())
        ), //)
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
            .wrap(build_cors(&AppConfig::default()))
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .configure(http_public::configure)
                    .configure(http_protected::configure),
            )
        //.configure(configure)
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
