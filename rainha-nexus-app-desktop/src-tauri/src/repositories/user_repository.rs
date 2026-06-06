use sqlx::{Pool, Sqlite};

use crate::models::user_model::{User, UserSend};

pub async fn find_by_email(db: &Pool<Sqlite>, email: &str) -> Result<Option<User>, sqlx::Error> {
    let result = match sqlx::query_as::<_, User>(
        r#"
            SELECT id, name, email, password_hash, created_at, updated_at, active
            FROM users
            WHERE email = ?
        "#,
    )
    .bind(email)
    .fetch_optional(db)
    .await
    {
        Ok(result) => result,
        Err(err) => {
            println!("error: {err:?}");
            return Err(err);
        }
    };
    Ok(result)
}

// pub async fn get_by_id(db: &Pool<Sqlite>, id: &i64) -> Result<Option<User>, sqlx::Error> {
//     sqlx::query_as::<_, User>(
//         r#"
//             SELECT id, name, email, active
//             FROM users
//             WHERE id = ?
//         "#,
//     )
//     .bind(id)
//     .fetch_optional(db)
//     .await
// }

pub async fn get_all_users(db: &Pool<Sqlite>) -> Result<Vec<UserSend>, sqlx::Error> {
    sqlx::query_as::<_, UserSend>(
        r#"
            SELECT id, name, email, active, created_at, updated_at
            FROM users
        "#,
    )
    .fetch_all(db)
    .await
}

pub async fn add_user(
    db: &Pool<Sqlite>,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<String, sqlx::Error> {
    sqlx::query(
        r#"
            INSERT INTO users (name, email, password_hash)
            VALUES (?, ?, ?)
        "#,
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .execute(db)
    .await?;
    Ok("User created successfully".to_string())
}
