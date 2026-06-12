use serde::Serialize;

/// 系统信息
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct SystemInfo {
    pub os: String,
    pub cpu_count: usize,
    pub total_memory_gb: f64,
}

#[allow(dead_code)]
impl SystemInfo {
    pub fn gather() -> Self {
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        Self {
            os: std::env::consts::OS.to_string(),
            cpu_count,
            total_memory_gb: 0.0,
        }
    }
}