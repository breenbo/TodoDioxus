use api::Todo;
use dioxus::prelude::*;

#[component]
pub fn DisplayTodos() -> Element {
    let todos = use_context::<Resource<Result<Vec<Todo>, ServerFnError>>>();
    let todo_count = use_memo(move || todos().and_then(|r| r.ok()).map(|v| v.len()).unwrap_or(0));

    rsx! {
        div { "Total todos: {todo_count}" }
        match todos() {
            Some(Ok(list)) => rsx! {
                for todo in list {
                    div { "{todo.content}" }
                }
            },
            Some(Err(e)) => rsx! { div { "Error: {e}" } },
            None => rsx! { div { "Loading..." } },
        }
    }
}
