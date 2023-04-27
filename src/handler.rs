use crate::model::PostRequest;
use crate::repository::Repository;
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};

pub async fn get_posts(data: Data<Repository>) -> HttpResponse {
    let posts = data.get_posts();
    HttpResponse::Ok().json(&posts)
}

pub async fn get_post(path: Path<i32>, data: Data<Repository>) -> HttpResponse {
    let id = path.into_inner();
    let post = data.get_post(id);
    HttpResponse::Ok().json(post)
}

pub async fn create_post(json: Json<PostRequest>, data: Data<Repository>) -> HttpResponse {
    let post = json.into_inner();
    data.create_post(post);
    HttpResponse::Ok().finish()
}

pub async fn update_post(
    path: Path<i32>,
    json: Json<PostRequest>,
    data: Data<Repository>,
) -> HttpResponse {
    let id = path.into_inner();
    let post = json.into_inner();
    data.update_post(id, post);
    HttpResponse::Ok().finish()
}

pub async fn delete_post(path: Path<i32>, data: Data<Repository>) -> HttpResponse {
    let id = path.into_inner();
    data.delete_post(id);
    HttpResponse::Ok().finish()
}
