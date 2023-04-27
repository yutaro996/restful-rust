mod handler;
mod model;
mod repository;
mod router;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use repository::Repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = Repository::new("posts.db");
    tracing_subscriber::fmt::init();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(Data::new(repo.clone()))
            .configure(router::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
