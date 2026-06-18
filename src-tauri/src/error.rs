//! 统一错误处理
//!
//! 所有业务错误收敛到此处，前端通过字符串匹配处理。
//! 规范：
//! 1. 业务层返回 Result<T, AppError>
//! 2. Command 层将 AppError 转为 String 返回给前端

use std::fmt;

/// 应用级错误类型
#[derive(Debug)]
pub enum AppError {
    /// 内部错误
    Internal(String),
    /// 资源未找到
    NotFound(String),
    /// 未授权
    Unauthorized(String),
    /// 请求参数无效
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// 转换为 String 供 Tauri Command 直接返回
impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}