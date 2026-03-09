use api::Todo;
use dioxus::prelude::*;

use crate::todos::{todo_card::ToggleTodo, RemoveTodo};

#[component]
pub fn CardContainer(todo: Todo) -> Element {
    let id = todo.id;
    let mut hover = use_signal(|| false);

    rsx! {
        div {
            class:"relative flex justify-between bg-white mb-2 w-full rounded-lg p-2 border border-neutral-300 hover:border-neutral-500 hover:shadow-lg transition-all",
            onmouseenter: move |_| hover.set(true),
            onmouseleave: move |_| hover.set(false),
            div {
                {todo.content.clone()}
            }
            ToggleTodo { todo }

            if hover() {
                div {
                    class:"absolute top-1 right-1",
                    RemoveTodo { id }
                }
            }
        }
    }
}
