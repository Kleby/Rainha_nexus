use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

pub async fn get_connection() -> Result<Pool<Sqlite>, sqlx::Error> {
    // Define o caminho absoluto exato fornecido por você
    let db_path = "../database.db";

    // Configura o SQLx para apontar para esse caminho e criar o arquivo se ele não existir
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    SqlitePool::connect_with(options).await
}

pub async fn migrate(pool: &Pool<Sqlite>) -> Result<String, sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            deleted_at TIMESTAMP DEFAULT NULL,
            active INTEGER DEFAULT 1
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(String::from("Migration successful"))
}
