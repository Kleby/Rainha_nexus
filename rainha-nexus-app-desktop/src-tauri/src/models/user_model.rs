use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
    pub active: i8,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, Debug)]
pub struct UserRegister {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, Debug)]
pub struct UserSend {
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,

    pub active: i8,
}

#[derive(serde::Serialize, sqlx::FromRow, Debug)]
pub struct UserPartial {
    pub id: i64,
    pub name: String,
    pub email: String,
}
