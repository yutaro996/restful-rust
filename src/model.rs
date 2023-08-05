use rusqlite::{Error, Row};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
        })
    }
}

#[derive(Deserialize)]
pub struct PostRequest {
    pub title: String,
    pub content: String,
}
