//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

use crate::model::TodoSql;

#[cfg(feature = "server")]
pub mod db_init;
#[cfg(feature = "server")]
pub mod model;

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[get("/api/todos")]
pub async fn get_todo_list() -> Result<Vec<TodoSql>, ServerFnError> {
    let db = db_init::init_db().await;
    let rows: Vec<TodoSql> =
        sqlx::query_as::<_, TodoSql>("SELECT id, content, completed FROM todos")
            .fetch_all(db)
            .await
            .unwrap();

    let mut todos = Vec::new();

    for row in rows {
        let todo = TodoSql {
            id: row.id,
            content: row.content,
            completed: row.completed,
        };

        todos.push(todo);
    }

    Ok(todos)
}

#[get("/api/todos/{uuid}")]
pub async fn get_todo(uuid: i64) -> Result<TodoSql, ServerFnError> {
    let db = db_init::init_db().await;
    let row: TodoSql =
        sqlx::query_as::<_, TodoSql>("SELECT content, completed FROM todos WHERE id = $1")
            .bind(uuid)
            .fetch_one(db)
            .await
            .unwrap();

    let todo = TodoSql {
        id: uuid,
        content: row.content,
        completed: row.completed,
    };

    Ok(todo)
}

#[post("/api/todos")]
pub async fn add_todo(content: String, completed: bool) -> Result<i64, ServerFnError> {
    let db = db_init::init_db().await;
    let res = sqlx::query("INSERT INTO todos (content, completed) VALUES($1, $2)")
        .bind(&content)
        .bind(completed)
        .execute(db)
        .await
        .unwrap();

    Ok(res.last_insert_rowid())
}
