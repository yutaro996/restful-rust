mod handler;
mod model;
mod repository;

use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = repository::Repository::new("posts.db");
    tracing_subscriber::fmt::init();
    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(repo.clone()))
            .service(handler::get_posts)
            .service(handler::get_post)
            .service(handler::create_post)
            .service(handler::update_post)
            .service(handler::delete_post)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
