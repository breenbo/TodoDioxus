use api::{toggle_todo, Todo};
use dioxus::prelude::*;

#[component]
pub fn ToggleTodo(todo: Todo, completed: Signal<bool>) -> Element {
    // use optimistic update of a local value
    let toggle = move |_| async move {
        *completed.write() = !completed();
        // get back if BE error
        if toggle_todo(todo.id).await.is_err() {
            *completed.write() = !completed();
        }
    };

    rsx! {
        div {
            class: "text-center w-16  px-2 py-1 mr-6 cursor-pointer rounded-sm",
            class: if completed() {"bg-green-200"} else {"bg-red-200"},
            onclick: toggle,
            if completed() {"Done"} else {"Todo"}
        }
    }
}
