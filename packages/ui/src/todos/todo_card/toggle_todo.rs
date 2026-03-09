use api::{toggle_todo, Todo};
use dioxus::prelude::*;

#[component]
pub fn ToggleTodo(todo: Todo) -> Element {
    let mut completed = use_signal(|| todo.completed);
    let toggle = move |_| async move {
        *completed.write() = !completed();
        if toggle_todo(todo.id).await.is_err() {
            *completed.write() = completed();
        }
    };

    rsx! {
        if completed() {
            div {
                class: "bg-green-200",
                onclick: toggle,
                "Done"
            }
        } else {
            div {
                class: "bg-red-200",
                onclick: toggle,
                "Todo"
            }

        }
    }
}
