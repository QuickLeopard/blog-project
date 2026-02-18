use actix_web::{web, App, HttpServer, HttpResponse, Result};

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use infrastructure::database::{create_pool, run_migrations};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/blog".to_string());

    let pool = create_pool(&database_url).await.expect("Failed to create pool");
    
    run_migrations(&pool).await.expect("Failed to run migrations");

    let server_address = "0.0.0:8080";

    println! ("Starting server at http://{server_address}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            //.route("/accounts/{id}", web::get().to(get_account_handler))
    })
    .bind(&server_address)?
    .run()
    .await
} 
