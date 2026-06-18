// ==================== 模块注册 ====================
mod commands;
mod error;
mod models;
mod services;
mod state;
mod tray;

use state::AppState;

/// Tauri 应用初始化与运行
///
/// 职责：
/// 1. 注册插件
/// 2. 注册全局状态
/// 3. 注册所有 Tauri Command
/// 4. 初始化系统托盘
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        // ----- 插件 -----
        .plugin(tauri_plugin_opener::init())
        // ----- 全局状态 -----
        .manage(app_state)
        // ----- Command 注册 -----
        // 每新增一个 command 模块，在此处添加一行即可
        .invoke_handler(tauri::generate_handler![
            commands::auth::greet,
            commands::file::file_list,
            commands::system::system_info,
        ])
        // ----- 初始化 -----
        .setup(|app| {
            tray::setup(app)?;
            Ok(())
        })
        // ----- 启动 -----
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}