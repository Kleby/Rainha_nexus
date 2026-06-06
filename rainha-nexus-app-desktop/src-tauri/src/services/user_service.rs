use sqlx::{Pool, Sqlite};

use crate::{models::user_model::UserSend, repositories::user_repository::get_all_users};

pub async fn get_all_users_service(pool: &Pool<Sqlite>) -> Result<Vec<UserSend>, sqlx::Error> {
    let users = get_all_users(pool).await;
    println!("{:?}", users);
    users
}
