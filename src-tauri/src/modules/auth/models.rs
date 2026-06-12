use serde::{Deserialize, Serialize};

/// 用户信息模型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
}

/// 登录请求
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}
