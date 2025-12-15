//! 环境变量读取工具模块

use std::net::IpAddr;

/// 从环境变量读取布尔值
pub fn env_bool(key: &str, default: bool) -> bool {
    match std::env::var(key) {
        Ok(raw) => {
            let v = raw.trim().to_ascii_lowercase();
            matches!(v.as_str(), "1" | "true" | "yes" | "y" | "on")
        }
        Err(_) => default,
    }
}

/// 从环境变量读取字符串
pub fn env_string(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// 从环境变量读取 u16
pub fn env_u16(key: &str) -> Option<u16> {
    env_string(key).and_then(|s| s.parse::<u16>().ok())
}

/// 从环境变量读取 u32
pub fn env_u32(key: &str) -> Option<u32> {
    env_string(key).and_then(|s| s.parse::<u32>().ok())
}

/// 从环境变量读取 IP 地址
pub fn env_ip(key: &str) -> Option<IpAddr> {
    env_string(key).and_then(|s| s.parse::<IpAddr>().ok())
}
