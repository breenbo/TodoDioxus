use serde::{Deserialize, Serialize};

// struct
#[cfg(feature = "server")]
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct TodoSql {
    pub id: i64,
    pub content: String,
    pub completed: bool,
}
