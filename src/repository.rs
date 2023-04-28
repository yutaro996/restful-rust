use crate::error::ApiError;
use crate::model::{Post, PostRequest};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

#[derive(Clone)]
pub struct Repository {
    pool: Pool<SqliteConnectionManager>,
}

impl Repository {
    pub fn new(path: &str) -> Result<Self, ApiError> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        let conn = pool.get()?;
        let mut stmt = conn.prepare(
            "CREATE TABLE IF NOT EXISTS post (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title STRING NOT NULL,
                    body STRING NOT NULL
                )",
        )?;
        stmt.execute(())?;
        Ok(Repository { pool })
    }

    pub fn select_all_posts(&self) -> Result<Vec<Post>, ApiError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, title, body FROM post")?;
        let posts = stmt.query_map((), Post::from_row)?.flatten().collect();
        Ok(posts)
    }

    pub fn select_post_with_id(&self, id: i32) -> Result<Post, ApiError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, title, body FROM post WHERE id = (?1)")?;
        let post = stmt.query_row(params![id], Post::from_row)?;
        Ok(post)
    }

    pub fn insert_new_post(&self, post: PostRequest) -> Result<(), ApiError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("INSERT INTO post (title, body) VALUES (?1, ?2)")?;
        stmt.execute(params![post.title, post.content])?;
        Ok(())
    }

    pub fn update_post_with_id(&self, id: i32, post: PostRequest) -> Result<(), ApiError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("UPDATE post SET title = (?1), body = (?2) WHERE id = (?3)")?;
        stmt.execute(params![post.title, post.content, id])?;
        Ok(())
    }

    pub fn delete_post_with_id(&self, id: i32) -> Result<(), ApiError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("DELETE FROM post WHERE id = (?1)")?;
        stmt.execute(params![id])?;
        Ok(())
    }
}
