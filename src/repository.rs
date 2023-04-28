use crate::model::{Post, PostRequest};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

#[derive(Clone)]
pub struct Repository {
    pool: Pool<SqliteConnectionManager>,
}

impl Repository {
    pub fn new(path: &str) -> Self {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager).unwrap();
        let conn = pool.get().unwrap();
        let mut stmt = conn
            .prepare(
                "CREATE TABLE IF NOT EXISTS post (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title STRING NOT NULL,
                    body STRING NOT NULL
                )",
            )
            .unwrap();
        stmt.execute(()).unwrap();
        Repository { pool }
    }

    pub fn select_all_posts(&self) -> Vec<Post> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, body FROM post").unwrap();
        let posts = stmt
            .query_map((), Post::from_row)
            .unwrap()
            .flatten()
            .collect();
        posts
    }

    pub fn select_post_with_id(&self, id: i32) -> Post {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, title, body FROM post WHERE id = (?1)")
            .unwrap();
        let post = stmt.query_row(params![id], Post::from_row).unwrap();
        post
    }

    pub fn insert_new_post(&self, post: PostRequest) {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn
            .prepare("INSERT INTO post (title, body) VALUES (?1, ?2)")
            .unwrap();
        stmt.execute(params![post.title, post.content]).unwrap();
    }

    pub fn update_post_with_id(&self, id: i32, post: PostRequest) {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn
            .prepare("UPDATE post SET title = (?1), body = (?2) WHERE id = (?3)")
            .unwrap();
        stmt.execute(params![post.title, post.content, id]).unwrap();
    }

    pub fn delete_post_with_id(&self, id: i32) {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("DELETE FROM post WHERE id = (?1)").unwrap();
        stmt.execute(params![id]).unwrap();
    }
}
