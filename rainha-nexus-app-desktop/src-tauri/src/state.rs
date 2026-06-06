use sqlx::{Pool, Sqlite};
use std::sync::Mutex;
pub struct AppState {
    // Poll de conexão dosqlite
    pub db: Pool<Sqlite>,
    pub session: Mutex<Option<UseSession>>,
}

#[derive(Clone)]
pub struct UseSession {
    pub id: i64,
    pub name: String,
    pub email: String,
}
