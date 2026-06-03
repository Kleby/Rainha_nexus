use sqlx::{sqlite::SqlitePoolOptions, Sqlite, Pool};

pub async fn get_connect()-> Result<Pool<Sqlite>, sqlx::Error> {
    Ok(SqlitePoolOptions::new().connect("sqlite://database.db").await?)
}

pub async fn migrate(pool:&Pool<Sqlite>) -> Result<({message: String, status:bool}), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            CONSTRAINT users_email_unique UNIQUE (email),
            CONSTRAINT users_name_unique UNIQUE (name)
        );
        "#
    ).execute(pool).await?;
    Ok(({message: String::from("Migration successful"), status: true}))

}
