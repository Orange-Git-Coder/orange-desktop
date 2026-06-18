//! 文件 Command
//!
//! 文件相关的 Tauri 命令入口

use crate::services::file_service::FileService;

/// 获取文件列表
#[tauri::command]
pub fn file_list() -> Result<String, String> {
    let service = FileService::new();
    service.list_files()
}