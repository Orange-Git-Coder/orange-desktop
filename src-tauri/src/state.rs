//! 全局状态管理
//!
//! 存放跨模块共享的状态，通过 `tauri::State<'_, AppState>` 注入到 Command 中。
//! 业务模块的独立状态不应放在此处，而是放在对应 Service 中。

use std::sync::atomic::{AtomicU64, Ordering};

/// 应用全局状态
pub struct AppState {
    /// 示例：访客计数器
    visitor_count: AtomicU64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            visitor_count: AtomicU64::new(0),
        }
    }

    /// 递增并返回新值
    pub fn increment_visitor(&self) -> u64 {
        self.visitor_count.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// 获取当前值
    #[allow(dead_code)]
    pub fn visitor_count(&self) -> u64 {
        self.visitor_count.load(Ordering::Relaxed)
    }
}