use api::Todo;
use dioxus::prelude::*;

use crate::todos::{todo_card::ToggleTodo, RemoveTodo};

#[component]
pub fn CardContainer(todo: Todo) -> Element {
    let id = todo.id;
    let mut hover = use_signal(|| false);
    let completed = use_signal(|| todo.completed);

    rsx! {
        div {
            class:"relative flex items-center justify-between bg-white mb-2 w-full rounded-lg p-2 border border-neutral-300 hover:border-neutral-500 hover:shadow-lg transition-all",
            onmouseenter: move |_| hover.set(true),
            onmouseleave: move |_| hover.set(false),
            div {
                class: if completed() { "line-through text-neutral-200"},
                {todo.content.clone()}
            }
            ToggleTodo { todo, completed }

            if hover() {
                div {
                    class:"absolute top-1 right-1",
                    RemoveTodo { id }
                }
            }
        }
    }
}
