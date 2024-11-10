//! App stuff. Not much exists here yet.

#![allow(non_snake_case)]

use dioxus::prelude::*;
// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::views::Route;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    // Allow unused qualifications because this confuses Clippy
    #[allow(unused_qualifications)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

/// Render the App
///
/// This function only really renders `Router::<Route> {}`. If you want to see where the real magic happens, go look at `views::Route`.
pub(crate) fn App() -> Element {
    rsx! { Router::<Route> {} }
}
