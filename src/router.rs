use crate::handler;
use actix_web::web;

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .route("/posts", web::get().to(handler::get_posts))
        .route("/posts/{id}", web::get().to(handler::get_post))
        .route("/posts", web::post().to(handler::create_post))
        .route("/posts/{id}", web::patch().to(handler::update_post))
        .route("/posts/{id}", web::delete().to(handler::delete_post));
}
