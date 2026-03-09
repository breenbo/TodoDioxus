#[cfg(feature = "server")]
use sqlx::sqlite::SqliteConnectOptions;
#[cfg(feature = "server")]
use sqlx::{Pool, Sqlite};
#[cfg(feature = "server")]
use std::str::FromStr;
#[cfg(feature = "server")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

// fn to connect to the db
#[cfg(feature = "server")]
async fn connect_db() -> Pool<Sqlite> {
    // NOTE: read env file and get the db url
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // NOTE: connect to the db with options
    let options = SqliteConnectOptions::from_str(&db_url)
        .expect("invalid DATABASE_URL")
        .create_if_missing(true);

    let pool = sqlx::sqlite::SqlitePool::connect_with(options)
        .await
        .expect("Failed to connect to db");

    // NOTE: move to migrations folder and files
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

#[cfg(feature = "server")]
pub async fn init_db() -> &'static Pool<Sqlite> {
    DB.get_or_init(connect_db).await
}
