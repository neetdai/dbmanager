mod menu_build;
mod db;
mod ssh_client;

use db::config::*;
use menu_build::build_menu;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            build_menu(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            test_connection,
            save_config,
            get_db_configs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
