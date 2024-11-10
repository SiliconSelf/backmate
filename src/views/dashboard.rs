//! The Dashboard view where the main app interactions will go

use dioxus::prelude::*;

/// Render the dashboard view
#[allow(non_snake_case)]
pub(crate) fn Dashboard() -> Element {
    rsx! { h1 {"Dashboard View"} }
}
