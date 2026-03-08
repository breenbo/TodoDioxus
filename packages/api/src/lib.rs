//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
pub mod db_init;
#[cfg(feature = "server")]
pub mod model;
#[cfg(feature = "server")]
use model::TodoSql;

/// Shared Todo type for client and server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: i64,
    pub content: String,
    pub completed: bool,
}

/// Echo the user input on the server.
#[server]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[get("/api/todos")]
pub async fn get_todo_list() -> Result<Vec<Todo>, ServerFnError> {
    let db = db_init::init_db().await;
    let rows: Vec<TodoSql> =
        sqlx::query_as::<_, TodoSql>("SELECT id, content, completed FROM todos")
            .fetch_all(db)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|r| Todo {
            id: r.id,
            content: r.content,
            completed: r.completed,
        })
        .collect())
}

#[get("/api/todos/{uuid}")]
pub async fn get_todo(uuid: i64) -> Result<Todo, ServerFnError> {
    let db = db_init::init_db().await;
    let row: TodoSql =
        sqlx::query_as::<_, TodoSql>("SELECT id, content, completed FROM todos WHERE id = $1")
            .bind(uuid)
            .fetch_one(db)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(Todo {
        id: row.id,
        content: row.content,
        completed: row.completed,
    })
}

#[post("/api/todos")]
pub async fn add_todo_to_db(content: String) -> Result<i64, ServerFnError> {
    let db = db_init::init_db().await;
    let res = sqlx::query("INSERT INTO todos (content, completed) VALUES($1, $2)")
        .bind(&content)
        .bind(false)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(res.last_insert_rowid())
}

#[delete("/api/todos/{uuid}")]
pub async fn delete_todo_from_db(uuid: i64) -> Result<(), ServerFnError> {
    let db = db_init::init_db().await;

    sqlx::query("DELETE FROM todos WHERE id=$1")
        .bind(uuid)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
