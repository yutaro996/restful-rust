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
        pool.get()
            .unwrap()
            .prepare(
                "CREATE TABLE IF NOT EXISTS post (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title STRING NOT NULL,
                    body STRING NOT NULL
                )",
            )
            .unwrap()
            .execute(())
            .unwrap();
        Repository { pool }
    }

    pub fn get_posts(&self) -> Vec<Post> {
        self.pool
            .get()
            .unwrap()
            .prepare("SELECT id, title, body FROM post")
            .unwrap()
            .query_map((), |row| {
                Ok(Post {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    body: row.get(2).unwrap(),
                })
            })
            .unwrap()
            .flatten()
            .collect()
    }

    pub fn get_post(&self, id: i32) -> Post {
        self.pool
            .get()
            .unwrap()
            .prepare("SELECT id, title, body FROM post WHERE id = (?1)")
            .unwrap()
            .query_row(params![id], |row| {
                Ok(Post {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    body: row.get(2).unwrap(),
                })
            })
            .unwrap()
    }

    pub fn create_post(&self, post: PostRequest) {
        self.pool
            .get()
            .unwrap()
            .prepare("INSERT INTO post (title, body) VALUES (?1, ?2)")
            .unwrap()
            .execute(params![post.title, post.body])
            .unwrap();
    }

    pub fn update_post(&self, id: i32, post: PostRequest) {
        self.pool
            .get()
            .unwrap()
            .prepare("UPDATE post SET title = (?1), body = (?2) WHERE id = (?3)")
            .unwrap()
            .execute(params![post.title, post.body, id])
            .unwrap();
    }

    pub fn delete_post(&self, id: i32) {
        self.pool
            .get()
            .unwrap()
            .prepare("DELETE FROM post WHERE id = (:id)")
            .unwrap()
            .execute(params![id])
            .unwrap();
    }
}
