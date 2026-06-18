//! 文件服务

use crate::error::AppError;

/// 文件信息
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
}

/// 文件系统服务
pub struct FileService;

impl FileService {
    pub fn new() -> Self {
        Self
    }

    /// 获取文件列表（示例实现）
    pub fn list_files(&self) -> Result<String, String> {
        Ok("File system ready".into())
    }

    /// 删除文件（示例）
    #[allow(dead_code)]
    pub fn delete_file(&self, path: &str) -> Result<(), AppError> {
        if path.is_empty() {
            return Err(AppError::BadRequest("path is empty".into()));
        }
        Ok(())
    }
}