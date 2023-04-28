use crate::model::PostRequest;
use crate::repository::Repository;
use actix_web::{web, HttpResponse};

pub async fn get_posts(repo: web::Data<Repository>) -> HttpResponse {
    let posts = repo.select_all_posts();
    HttpResponse::Ok().json(posts)
}

pub async fn get_post(id: web::Path<i32>, repo: web::Data<Repository>) -> HttpResponse {
    let post = repo.select_post_with_id(id.into_inner());
    HttpResponse::Ok().json(post)
}

pub async fn create_post(
    post: web::Json<PostRequest>,
    repo: web::Data<Repository>,
) -> HttpResponse {
    repo.insert_new_post(post.into_inner());
    HttpResponse::Ok().finish()
}

pub async fn update_post(
    id: web::Path<i32>,
    post: web::Json<PostRequest>,
    repo: web::Data<Repository>,
) -> HttpResponse {
    repo.update_post_with_id(id.into_inner(), post.into_inner());
    HttpResponse::Ok().finish()
}

pub async fn delete_post(id: web::Path<i32>, repo: web::Data<Repository>) -> HttpResponse {
    repo.delete_post_with_id(id.into_inner());
    HttpResponse::Ok().finish()
}
