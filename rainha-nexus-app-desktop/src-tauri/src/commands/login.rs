use crate::{
    models::session_model::SessionResponse,
    services::auth_service,
    state::{AppState, UseSession},
};
use tauri::State;

#[tauri::command]
pub async fn login(
    state: State<'_, AppState>,
    email: &str,
    password: &str,
) -> Result<SessionResponse, String> {
    let user = auth_service::login(&state.db, &email, &password).await?;
    let session = UseSession {
        id: user.id,
        name: user.name.clone(),
        email: user.email.clone(),
    };
    *state.session.lock().unwrap() = Some(session);

    Ok(SessionResponse {
        id: user.id,
        name: user.name.clone(),
        email: user.email.clone(),
    })
}
