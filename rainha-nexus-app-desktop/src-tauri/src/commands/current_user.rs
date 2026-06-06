use tauri::State;

use crate::{models::session_model::SessionResponse, state::AppState};
#[tauri::command]

pub async fn current_user(state: State<'_, AppState>) -> Result<SessionResponse, String> {
    let guard = state.session.lock().unwrap();
    guard.as_ref().map(|user| SessionResponse {
        id: user.id,
        name: user.name.clone(),
        email: user.email.clone(),
    });
    Ok(SessionResponse::default())
}
