use sqlx::{Pool, Sqlite};

use crate::{
    models::user_model::{User, UserRegister},
    repositories::user_repository::{add_user, find_by_email},
    services::password_service::{hash_password, verify_password},
};

pub async fn register(db: &Pool<Sqlite>, user: &UserRegister) -> Result<(), String> {
    let existing_user = find_by_email(db, &user.email)
        .await
        .map_err(|_| "Email already in use".to_string())?;

    if existing_user.is_some() {
        return Err("Email already in use".into());
    }
    let password_hash = hash_password(&user.password);

    let result = add_user(db, &user.name, &user.email, &password_hash).await;
    Ok(())
}

pub async fn login(db: &Pool<Sqlite>, email: &str, password: &str) -> Result<User, String> {
    let user = find_by_email(db, email)
        .await
        .map_err(|_| "User not found".to_string())?;
    let user = match user {
        Some(user) => user,
        None => return Err("User not found".to_string()),
    };

    let valid_password = verify_password(password, &user.password_hash);
    if !valid_password {
        return Err("Invalid password".to_string());
    }
    Ok(user)
}
