use std::io::Write;

/// 简易应用日志器
pub struct Logger;

impl Logger {
    pub fn init() {
        println!("[orange-desktop] Logger initialized");
    }

    pub fn info(message: &str) {
        let _ = writeln!(std::io::stdout(), "[INFO] {}", message);
    }

    pub fn warn(message: &str) {
        let _ = writeln!(std::io::stderr(), "[WARN] {}", message);
    }

    pub fn error(message: &str) {
        let _ = writeln!(std::io::stderr(), "[ERROR] {}", message);
    }
}