use dioxus::prelude::*;
use ui::Hero;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        div {
            class:"h-20 w-full bg-green-300",
            "Component from the web package with tailwindcss"
        }
    }
}
