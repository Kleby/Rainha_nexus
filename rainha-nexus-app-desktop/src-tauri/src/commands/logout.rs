use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<(), String> {
    *state.session.lock().await = None;
    Ok(())
}
