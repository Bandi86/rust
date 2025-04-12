running project with dx serve
address: http://127.0.0.1:8080

basics main.rs

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "HotDog!" }
}