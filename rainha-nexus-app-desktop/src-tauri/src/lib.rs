// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod models;
mod state;
mod database;

use database::connection::{get_connection, migrate};
use state::AppState;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::block_on(async {
        let pool = get_connection().await.expect("Erro ao conectar com o banco de dados!");
        migrate(&pool).await.expect("Erro ao migrar o banco de dados para criar tabelas!");
    });

    tauri::Builder::default()
        .manage(AppState{
            db: pool,
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
