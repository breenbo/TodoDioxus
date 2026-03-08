use api::Todo;
use dioxus::prelude::*;

#[component]
pub fn CardContainer(todo: Todo) -> Element {
    rsx! {
        div {
            class:"bg-white mb-2 w-full rounded-lg p-2 border border-neutral-300 hover:border-neutral-500 hover:shadow-lg transition-all",
            {todo.content}
        }
    }
}
