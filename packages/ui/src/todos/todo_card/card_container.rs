use api::Todo;
use dioxus::prelude::*;

use crate::todos::RemoveTodo;

#[component]
pub fn CardContainer(todo: Todo) -> Element {
    let id = todo.id;

    rsx! {
        div {
            class:"flex justify-between bg-white mb-2 w-full rounded-lg p-2 border border-neutral-300 hover:border-neutral-500 hover:shadow-lg transition-all",
            div {
                {todo.content}
            }
            div {
                RemoveTodo { id }
            }
        }
    }
}
