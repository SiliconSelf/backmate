//! Views the user can be presented with
//!
//! The star of the show for this module is the `Route` enum that manages all
//! the possible app frontend views.

mod dashboard;
mod login;

pub(crate) use dashboard::Dashboard;
use dioxus::prelude::*;
pub(crate) use login::Login;

/// Possible views the user can be presented with
#[derive(Routable, PartialEq, Clone)]
pub(crate) enum Route {
    /// The login page
    #[route("/")]
    // Redirect any weird states that shouldn't happen back to the login page
    // just in case
    #[route("/:..segments")]
    Login {},
    /// The dashboard page
    #[route("/dashboard")]
    Dashboard {},
}
