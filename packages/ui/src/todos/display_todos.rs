use api::Todo;
use dioxus::prelude::*;

use crate::todos::CardContainer;

#[component]
pub fn DisplayTodos() -> Element {
    let todos = use_context::<Resource<Result<Vec<Todo>, ServerFnError>>>();
    let todo_count = use_memo(move || todos().and_then(|r| r.ok()).map(|v| v.len()).unwrap_or(0));

    rsx! {
        div {
            class:"pb-4",
            "Total todos: {todo_count}"
        }
        match todos() {
            Some(Ok(list)) => rsx! {
                for todo in list {
                    CardContainer { todo }
                }
            },
            Some(Err(e)) => rsx! { div { "Error: {e}" } },
            None => rsx! { div { "Loading..." } },
        }
    }
}
