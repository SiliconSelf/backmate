mod login;
mod dashboard;

use dioxus::prelude::*;

pub(crate) use login::Login;
pub(crate) use dashboard::Dashboard;

#[derive(Routable, PartialEq, Clone)]
pub(crate) enum Route {
    #[route("/")]
    #[route("/:..segments")]
    Login {},
    #[route("/dashboard")]
    Dashboard {}
}