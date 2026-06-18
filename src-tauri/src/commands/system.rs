//! 系统 Command
//!
//! 系统信息相关的 Tauri 命令入口

use crate::services::system_service::SystemService;

/// 获取系统信息
#[tauri::command]
pub fn system_info() -> Result<String, String> {
    let service = SystemService::new();
    let info = service.gather_info();
    serde_json::to_string(&info).map_err(|e| e.to_string())
}