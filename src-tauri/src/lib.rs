mod core;
mod modules;
mod utils;

use crate::core::AppState;

/// 托盘图标设置（保留原有功能）
mod tray;
pub use tray::setup as setup_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            modules::auth::commands::greet,
        ])
        .setup(|app| {
            tray::setup(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}