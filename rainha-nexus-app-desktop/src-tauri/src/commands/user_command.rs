use crate::models::user_model::{User, UserSend};
use tauri::State;

use crate::{
    services::{auth_service, user_service::get_all_users_service},
    AppState,
};

#[tauri::command]
pub async fn get_all(state: State<'_, AppState>) -> Result<Vec<UserSend>, String> {
    let users = get_all_users_service(&state.db).await;
    match users {
        Ok(users) => Ok(users),
        Err(err) => Err(err.to_string()),
    }
}
