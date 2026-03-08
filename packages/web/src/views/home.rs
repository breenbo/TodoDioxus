use dioxus::prelude::*;

use api::get_todo_list;
use ui::{AddTodo, CenterContainer, DisplayTodos, Hero};

#[component]
pub fn Home() -> Element {
    // get the todos here and make it reachable by childs
    let todos = use_resource(|| async move { get_todo_list().await });
    use_context_provider(|| todos);

    rsx! {
        CenterContainer {
            Hero {},
            AddTodo {},
            DisplayTodos {}
        }
    }
}
