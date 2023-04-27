use crate::handler::{create_post, delete_post, get_post, get_posts, update_post};
use actix_web::web::{delete, get, patch, post, ServiceConfig};

pub fn configure(config: &mut ServiceConfig) {
    config
        .route("/posts", get().to(get_posts))
        .route("/posts/{id}", get().to(get_post))
        .route("/posts", post().to(create_post))
        .route("/posts/{id}", patch().to(update_post))
        .route("/posts/{id}", delete().to(delete_post));
}
