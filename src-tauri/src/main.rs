// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// 应用启动入口
///
/// 职责：
/// - 仅负责启动 Tauri 应用
/// - 不包含任何业务逻辑
fn main() {
    orange_desktop_lib::run()
}