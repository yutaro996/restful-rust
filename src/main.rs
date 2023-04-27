mod handler;
mod model;
mod repository;
mod router;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use repository::Repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let repository = Repository::new(&database_url);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(repository.clone()))
            .configure(router::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
