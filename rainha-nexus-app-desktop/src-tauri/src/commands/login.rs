mod models;
use models::user_model::User;


#[tauri::command]
pub async fn login() -> Result<(), String>{
    Ok(())
}

#[tauri::command]
pub async fn logout(user: User) -> Result<(), String>{
    Ok(())
}
