use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct PostRequest {
    pub title: String,
    pub body: String,
}
