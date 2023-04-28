mod handler;
mod model;
mod repository;
mod router;

use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let repo = repository::Repository::new(&database_url);
    tracing_subscriber::fmt::init();
    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::default())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(repo.clone()))
            .configure(router::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
