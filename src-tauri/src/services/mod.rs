//! Service 层
//!
//! 每个子模块封装一个业务领域的核心逻辑。
//! 职责：
//! 1. 实现具体业务逻辑
//! 2. 调用系统 API / 数据库 / 文件系统
//! 3. 返回业务友好类型
pub mod file_service;
pub mod system_service;