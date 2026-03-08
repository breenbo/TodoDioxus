#[cfg(feature = "server")]
use sqlx::{Pool, Sqlite};
#[cfg(feature = "server")]
use tokio::sync::OnceCell;

// initiate once the db in this var
#[cfg(feature = "server")]
static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

// fn to connect to the db
#[cfg(feature = "server")]
pub async fn connect_db() -> Pool<Sqlite> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite://db.sqlite")
        .await
        .expect("Failed to connect to db");

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    content TEXT NOT NULL
                    completed INTEGER
                )
    ",
    )
    .execute(&pool)
    .await
    .expect("Failed to create todo table");

    pool
}

// init the db
#[cfg(feature = "server")]
pub async fn init_db() -> &'static Pool<Sqlite> {
    DB.get_or_init(connect_db).await
}
