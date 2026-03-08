#[cfg(feature = "server")]
use sqlx::{Pool, Sqlite};
#[cfg(feature = "server")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

// fn to connect to the db
#[cfg(feature = "server")]
async fn connect_db() -> Pool<Sqlite> {
    use sqlx::sqlite::SqliteConnectOptions;

    let options = SqliteConnectOptions::new()
        .filename("db.sqlite")
        .create_if_missing(true);

    let pool = sqlx::sqlite::SqlitePool::connect_with(options)
        .await
        .expect("Failed to connect to db");

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS todos (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    content TEXT NOT NULL,
                    completed INTEGER NOT NULL DEFAULT 0
                )
    ",
    )
    .execute(&pool)
    .await
    .expect("Failed to create todo table");

    pool
}

#[cfg(feature = "server")]
pub async fn init_db() -> &'static Pool<Sqlite> {
    DB.get_or_init(connect_db).await
}
