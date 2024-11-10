#![doc = include_str!("../README.md")]

use tauri::{Manager, State};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {name}! You've been greeted from Rust!")
// }

/// Test if the user is currently logged in.
///
/// This is currently a testing function with no real functionality
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
fn logged_in(state: State<'_, Auth>) -> bool {
    state.logged_in
}

/// A struct for managing state regarding user authentication
pub struct Auth {
    /// If the user is logged in or not
    pub logged_in: bool,
}

/// Run the tauri application
///
/// # Panics
/// This function will panic if the graphical program fails to start (i.e. if
/// run on a headless system).
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![logged_in])
        .setup(|app| {
            app.manage(Auth {
                logged_in: false,
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
