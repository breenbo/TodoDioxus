use dioxus::prelude::*;

#[component]
pub fn CenterContainer(children: Element) -> Element {
    rsx! {
        div {
            class: "w-full",
            div {
                class: "max-w-2xl mx-auto",
                {children}
            }
        }
    }
}
