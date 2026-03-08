use dioxus::prelude::*;

// const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "h-20 w-full bg-red-200",
            "Component from Ui package"
        }
    }
}
