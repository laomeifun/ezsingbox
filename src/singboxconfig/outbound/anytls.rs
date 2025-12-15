use serde::{Deserialize, Serialize};

use crate::singboxconfig::shared::{DialFields, OutboundTlsConfig};
use crate::singboxconfig::types::Duration;

//============================================================================
// AnyTLS 出站配置
// ============================================================================

/// AnyTLS 出站配置
/// 自sing-box 1.12.0 起可用
/// 文档: https://sing-box.sagernet.org/configuration/outbound/anytls/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnyTlsOutbound {
    /// 出站类型，固定为 "anytls"
    #[serde(rename = "type")]
    pub outbound_type: String,

    /// 出站标签
    pub tag: String,

    /// 服务器地址（必填）
    pub server: String,

    /// 服务器端口（必填）
    pub server_port: u16,

    /// AnyTLS 密码（必填）
    pub password: String,

    /// 空闲会话检查间隔
    /// 默认值: 30s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_check_interval: Option<Duration>,

    /// 空闲会话超时时间
    /// 在检查中，关闭空闲时间超过此值的会话
    /// 默认值: 30s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_timeout: Option<Duration>,

    /// 最小空闲会话数
    /// 在检查中，至少保持前 n 个空闲会话打开
    /// 默认值: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_idle_session: Option<u32>,

    /// TLS 配置（必填）
    pub tls: OutboundTlsConfig,

    /// 拨号字段
    #[serde(flatten)]
    pub dial: DialFields,
}

impl AnyTlsOutbound {
    /// 创建新的 AnyTLS 出站配置
    pub fn new(
        tag: impl Into<String>,
        server: impl Into<String>,
        server_port: u16,
        password: impl Into<String>,
    ) -> Self {
        Self {
            outbound_type: "anytls".to_string(),
            tag: tag.into(),
            server: server.into(),
            server_port,
            password: password.into(),
            idle_session_check_interval: None,
            idle_session_timeout: None,
            min_idle_session: None,
            tls: OutboundTlsConfig::default(),
            dial: DialFields::default(),
        }
    }

    /// 设置空闲会话检查间隔
    pub fn with_idle_session_check_interval(mut self, interval: Duration) -> Self {
        self.idle_session_check_interval = Some(interval);
        self
    }

    /// 设置空闲会话超时时间
    pub fn with_idle_session_timeout(mut self, timeout: Duration) -> Self {
        self.idle_session_timeout = Some(timeout);
        self
    }

    /// 设置最小空闲会话数
    pub fn with_min_idle_session(mut self, min: u32) -> Self {
        self.min_idle_session = Some(min);
        self
    }

    /// 设置 TLS 配置
    pub fn with_tls(mut self, tls: OutboundTlsConfig) -> Self {
        self.tls = tls;
        self
    }

    /// 设置拨号字段
    pub fn with_dial(mut self, dial: DialFields) -> Self {
        self.dial = dial;
        self
    }
}

impl Default for AnyTlsOutbound {
    fn default() -> Self {
        Self {
            outbound_type: "anytls".to_string(),
            tag: String::new(),
            server: String::new(),
            server_port: 0,
            password: String::new(),
            idle_session_check_interval: None,
            idle_session_timeout: None,
            min_idle_session: None,
            tls: OutboundTlsConfig::default(),
            dial: DialFields::default(),
        }
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let outbound = AnyTlsOutbound::new("anytls-out", "127.0.0.1", 443, "password123");
        assert_eq!(outbound.outbound_type, "anytls");
        assert_eq!(outbound.tag, "anytls-out");
        assert_eq!(outbound.server, "127.0.0.1");
        assert_eq!(outbound.server_port, 443);
        assert_eq!(outbound.password, "password123");
    }

    #[test]
    fn test_serialize() {
        let outbound =
            AnyTlsOutbound::new("anytls-out", "example.com", 443, "8JCsPssfgS8tiRwiMlhARg==");
        let json = serde_json::to_string_pretty(&outbound).unwrap();
        assert!(json.contains("\"type\": \"anytls\""));
        assert!(json.contains("\"tag\": \"anytls-out\""));
        assert!(json.contains("\"server\": \"example.com\""));
        assert!(json.contains("\"server_port\": 443"));
        assert!(json.contains("\"password\": \"8JCsPssfgS8tiRwiMlhARg==\""));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "type": "anytls",
            "tag": "anytls-out",
            "server": "127.0.0.1",
            "server_port": 1080,
            "password": "test_password",
            "idle_session_check_interval": "30s",
            "idle_session_timeout": "30s",
            "min_idle_session": 5,
            "tls": {
                "enabled": true,
                "server_name": "example.com"
            }
        }"#;

        let outbound: AnyTlsOutbound = serde_json::from_str(json).unwrap();
        assert_eq!(outbound.outbound_type, "anytls");
        assert_eq!(outbound.tag, "anytls-out");
        assert_eq!(outbound.server, "127.0.0.1");
        assert_eq!(outbound.server_port, 1080);
        assert_eq!(outbound.password, "test_password");
        assert_eq!(outbound.min_idle_session, Some(5));
    }

    #[test]
    fn test_builder_pattern() {
        let outbound =
            AnyTlsOutbound::new("test", "server.com", 443, "pass").with_min_idle_session(3);

        assert_eq!(outbound.min_idle_session, Some(3));
    }
}
