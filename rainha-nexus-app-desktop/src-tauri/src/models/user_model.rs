use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    id: u64,
    username: String,
    email: String,
    password: String,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
    active: u8,
}
