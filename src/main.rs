mod error;
mod handler;
mod model;
mod repository;
mod router;

use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let repo = repository::Repository::new("post.db").unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .wrap(actix_cors::Cors::default())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(router::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
