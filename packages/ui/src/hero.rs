use dioxus::prelude::*;

// const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "w-full py-8 text-3xl text-center font-bold",
            "To Do's"
        }
    }
}
