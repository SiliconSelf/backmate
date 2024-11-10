mod dashboard;
mod login;

pub(crate) use dashboard::Dashboard;
use dioxus::prelude::*;
pub(crate) use login::Login;

#[derive(Routable, PartialEq, Clone)]
pub(crate) enum Route {
    #[route("/")]
    #[route("/:..segments")]
    Login {},
    #[route("/dashboard")]
    Dashboard {},
}
