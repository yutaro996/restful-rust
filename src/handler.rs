use crate::model::PostRequest;
use crate::repository::Repository;
use actix_web::{delete, get, patch, post, web, HttpResponse};

#[get("/posts")]
pub async fn get_posts(data: web::Data<Repository>) -> HttpResponse {
    let posts = data.get_posts();
    HttpResponse::Ok().json(&posts)
}

#[get("/posts/{id}")]
pub async fn get_post(path: web::Path<i32>, data: web::Data<Repository>) -> HttpResponse {
    let id = path.into_inner();
    let post = data.get_post(id);
    HttpResponse::Ok().json(post)
}

#[post("/posts")]
pub async fn create_post(
    json: web::Json<PostRequest>,
    data: web::Data<Repository>,
) -> HttpResponse {
    let post = json.into_inner();
    data.create_post(post);
    HttpResponse::Ok().finish()
}

#[patch("/posts/{id}")]
pub async fn update_post(
    path: web::Path<i32>,
    json: web::Json<PostRequest>,
    data: web::Data<Repository>,
) -> HttpResponse {
    let id = path.into_inner();
    let post = json.into_inner();
    data.update_post(id, post);
    HttpResponse::Ok().finish()
}

#[delete("/posts/{id}")]
pub async fn delete_post(path: web::Path<i32>, data: web::Data<Repository>) -> HttpResponse {
    let id = path.into_inner();
    data.delete_post(id);
    HttpResponse::Ok().finish()
}
