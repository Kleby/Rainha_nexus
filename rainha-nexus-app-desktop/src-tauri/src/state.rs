use sqlx::{Pool, Sqlite};
pub struct AppState{
// Poll de conexão dosqlite
    db: Pool<Sqlite>,
}
