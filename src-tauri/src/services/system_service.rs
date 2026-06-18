//! 系统信息服务

use serde::Serialize;

/// 系统信息
#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    /// 操作系统
    pub os: String,
    /// CPU 核心数
    pub cpu_count: usize,
}

/// 系统信息服务
pub struct SystemService;

impl SystemService {
    pub fn new() -> Self {
        Self
    }

    /// 收集系统信息
    pub fn gather_info(&self) -> SystemInfo {
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        SystemInfo {
            os: std::env::consts::OS.to_string(),
            cpu_count,
        }
    }
}