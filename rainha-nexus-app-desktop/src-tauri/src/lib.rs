// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod database;
mod models;
mod repositories;
mod services;
mod state;

use commands::{
    current_user::current_user, login::login, register::register, user_command::get_all,
};

use database::connection::{get_connection, migrate};
use state::AppState;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Retornamos a pool de dentro do bloco assíncrono para o escopo principal
    let pool = tauri::async_runtime::block_on(async {
        let pool = get_connection()
            .await
            .expect("Erro ao conectar com o banco de dados!");
        migrate(&pool)
            .await
            .expect("Erro ao migrar o banco de dados para criar tabelas!");
        pool // <-- Retorna a pool aqui
    });

    tauri::Builder::default()
        .manage(AppState {
            db: pool,
            session: std::sync::Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            login,
            register,
            get_all,
            current_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
