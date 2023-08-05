use crate::repository::Repository;
use crate::{error::ApiError, model::PostRequest};
use actix_web::{web, HttpResponse};

pub async fn get_posts(repo: web::Data<Repository>) -> Result<HttpResponse, ApiError> {
    let posts = repo.select_all_posts()?;
    Ok(HttpResponse::Ok().json(posts))
}

pub async fn get_post(
    id: web::Path<i32>,
    repo: web::Data<Repository>,
) -> Result<HttpResponse, ApiError> {
    let post = repo.select_post_with_id(id.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

pub async fn create_post(
    post: web::Json<PostRequest>,
    repo: web::Data<Repository>,
) -> Result<HttpResponse, ApiError> {
    repo.insert_new_post(post.into_inner())?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn update_post(
    id: web::Path<i32>,
    post: web::Json<PostRequest>,
    repo: web::Data<Repository>,
) -> Result<HttpResponse, ApiError> {
    repo.update_post_with_id(id.into_inner(), post.into_inner())?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_post(
    id: web::Path<i32>,
    repo: web::Data<Repository>,
) -> Result<HttpResponse, ApiError> {
    repo.delete_post_with_id(id.into_inner())?;
    Ok(HttpResponse::Ok().finish())
}
