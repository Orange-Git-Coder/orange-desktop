//! Command 模块
//!
//! 每个子模块对应一个业务领域。
//! 职责：接收前端调用 → 转发给 Service → 返回结果给前端
//! 严禁在 Command 中直接写业务逻辑。
pub mod auth;
pub mod file;
pub mod system;