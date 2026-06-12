use std::sync::atomic::{AtomicU64, Ordering};

/// 应用全局状态
pub struct AppState {
    visitor_count: AtomicU64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            visitor_count: AtomicU64::new(0),
        }
    }

    /// 递增访客计数并返回新值
    pub fn increment_visitor(&self) -> u64 {
        self.visitor_count.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// 获取当前访客计数
    #[allow(dead_code)]
    pub fn visitor_count(&self) -> u64 {
        self.visitor_count.load(Ordering::Relaxed)
    }
}