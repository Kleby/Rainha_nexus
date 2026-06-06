use crate::{models::user_model::UserRegister, services::auth_service, state::AppState};
use tauri::State;

#[tauri::command]
pub async fn register(state: State<'_, AppState>, user: UserRegister) -> Result<(), String> {
    println!("{user:?}");
    auth_service::register(&state.db, &user).await
}
