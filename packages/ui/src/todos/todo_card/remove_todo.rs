use api::{delete_todo_from_db, Todo};
use dioxus::prelude::*;

#[component]
pub fn RemoveTodo(id: i64) -> Element {
    let mut todos = use_context::<Resource<Result<Vec<Todo>, ServerFnError>>>();
    let remove = move |_| async move {
        let _ = delete_todo_from_db(id).await;
        todos.restart();
    };

    rsx! {
        div {
            class:"w-4 h-4 flex align-center justify-center text-xs rounded-full bg-red-500 text-white cursor-pointer",
            onclick: remove,
            "X"
        }
    }
}
