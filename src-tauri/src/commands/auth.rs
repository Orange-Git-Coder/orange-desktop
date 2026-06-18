//! 鉴权 Command
//!
//! 示例：问候命令

use crate::state::AppState;

/// 问候命令 —— 接收前端用户名，返回问候语
#[tauri::command]
pub fn greet(name: &str, state: tauri::State<'_, AppState>) -> String {
    let count = state.increment_visitor();
    format!("Hello, {}! 👋 You are visitor #{}", name, count)
}