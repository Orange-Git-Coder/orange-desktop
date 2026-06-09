use crate::state::AppState;

/// 问候命令 —— 接收前端传来的用户名，返回问候语
/// 用法: invoke('greet', { name: '小明' })
#[tauri::command]
pub fn greet(name: &str, state: tauri::State<'_, AppState>) -> String {
    let visitor_count = state.increment_visitor();
    format!(
        "Hello, {}! 👋 You are visitor #{}.",
        name, visitor_count
    )
}