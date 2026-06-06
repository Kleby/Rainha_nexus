use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, Default)]
pub struct SessionResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
}
