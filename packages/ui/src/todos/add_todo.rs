use api::{add_todo_to_db, Todo};
use dioxus::prelude::*;

#[component]
pub fn AddTodo() -> Element {
    let mut todos = use_context::<Resource<Result<Vec<Todo>, ServerFnError>>>();
    let mut content = use_signal(|| "".to_string());
    let add = move |_| async move {
        if content().is_empty() {
            return;
        }

        let _ = add_todo_to_db(content()).await;
        content.set("".to_string());
        todos.restart();
    };

    rsx! {
        div { class: "flex w-full justify-center gap-x-8 mb-10",
            input {
                class:"w-full border border-neutral-400 rounded-xl py-2 px-4",
                r#type: "text",
                placeholder: "Add a new todo",
                value: content(),
                oninput: move |e| content.set(e.value()),
                onkeydown: move |e| async move {
                    if e.key() == Key::Enter {
                        add(()).await;
                    }
                }
            }
            button {
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg",
                onclick: move |_| async move { add(()).await; },
                "Add"
            }
        }
    }
}
