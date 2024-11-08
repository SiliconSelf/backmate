use tauri::{Manager, State};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn logged_in(state: State<'_, Auth>) -> bool {
    state.logged_in
}

pub struct Auth {
    pub logged_in: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, logged_in])
        .setup(|app| {
            app.manage(Auth { logged_in: false });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
