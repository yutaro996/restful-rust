use crate::handler;
use actix_web::web;

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/posts")
            .service(
                web::resource("")
                    .route(web::get().to(handler::get_posts))
                    .route(web::post().to(handler::create_post)),
            )
            .service(
                web::scope("/{id}").service(
                    web::resource("")
                        .route(web::get().to(handler::get_post))
                        .route(web::patch().to(handler::update_post))
                        .route(web::delete().to(handler::delete_post)),
                ),
            ),
    );
}
