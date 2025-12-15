use serde::{Deserialize, Serialize};

use crate::singboxconfig::shared::{InboundTlsConfig, ListenFields};
use crate::singboxconfig::types::UserWithPassword;

//============================================================================
// AnyTLS 入站配置（服务端）
//============================================================================

/// AnyTLS 入站配置（服务端）
/// 自sing-box 1.12.0 起可用
/// 文档: https://sing-box.sagernet.org/configuration/inbound/anytls/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnyTlsInbound {
    /// 入站类型，固定为 "anytls"
    #[serde(rename = "type")]
    pub inbound_type: String,

    /// 入站标签
    pub tag: String,

    /// 监听字段
    #[serde(flatten)]
    pub listen: ListenFields,

    /// AnyTLS 用户列表（必填）
    pub users: Vec<UserWithPassword>,

    /// AnyTLS 填充方案行数组
    /// 用于混淆流量特征
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_scheme: Option<Vec<String>>,

    /// TLS 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<InboundTlsConfig>,
}

impl AnyTlsInbound {
    /// 创建新的 AnyTLS 入站配置
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            inbound_type: "anytls".to_string(),
            tag: tag.into(),
            listen: ListenFields::default(),
            users: Vec::new(),
            padding_scheme: None,
            tls: None,
        }
    }

    /// 添加用户
    pub fn add_user(mut self, name: impl Into<String>, password: impl Into<String>) -> Self {
        self.users.push(UserWithPassword::new(name, password));
        self
    }

    /// 设置监听地址
    pub fn with_listen(mut self, listen: impl Into<String>) -> Self {
        self.listen.listen = listen.into();
        self
    }

    /// 设置监听端口
    pub fn with_listen_port(mut self, port: u16) -> Self {
        self.listen.listen_port = Some(port);
        self
    }

    /// 设置填充方案
    pub fn with_padding_scheme(mut self, scheme: Vec<String>) -> Self {
        self.padding_scheme = Some(scheme);
        self
    }

    /// 使用默认填充方案
    pub fn with_default_padding_scheme(mut self) -> Self {
        self.padding_scheme = Some(Self::default_padding_scheme());
        self
    }

    /// 设置 TLS 配置
    pub fn with_tls(mut self, tls: InboundTlsConfig) -> Self {
        self.tls = Some(tls);
        self
    }

    /// 设置监听字段
    pub fn with_listen_fields(mut self, listen: ListenFields) -> Self {
        self.listen = listen;
        self
    }

    /// 获取默认填充方案
    /// 文档: https://sing-box.sagernet.org/configuration/inbound/anytls/
    pub fn default_padding_scheme() -> Vec<String> {
        vec![
            "stop=8".to_string(),
            "0=30-30".to_string(),
            "1=100-400".to_string(),
            "2=400-500,c,500-1000,c,500-1000,c,500-1000".to_string(),
            "3=9-9,500-1000".to_string(),
            "4=500-1000".to_string(),
            "5=500-1000".to_string(),
            "6=500-1000".to_string(),
            "7=500-1000".to_string(),
        ]
    }
}

impl Default for AnyTlsInbound {
    fn default() -> Self {
        Self {
            inbound_type: "anytls".to_string(),
            tag: String::new(),
            listen: ListenFields::default(),
            users: Vec::new(),
            padding_scheme: None,
            tls: None,
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
        let inbound = AnyTlsInbound::new("anytls-in");
        assert_eq!(inbound.inbound_type, "anytls");
        assert_eq!(inbound.tag, "anytls-in");
        assert!(inbound.users.is_empty());
    }

    #[test]
    fn test_add_user() {
        let inbound = AnyTlsInbound::new("anytls-in")
            .add_user("sekai", "8JCsPssfgS8tiRwiMlhARg==")
            .add_user("user2", "password2");

        assert_eq!(inbound.users.len(), 2);
        assert_eq!(inbound.users[0].name, "sekai");
        assert_eq!(inbound.users[0].password, "8JCsPssfgS8tiRwiMlhARg==");
        assert_eq!(inbound.users[1].name, "user2");
    }

    #[test]
    fn test_builder_pattern() {
        let inbound = AnyTlsInbound::new("anytls-in")
            .with_listen("::")
            .with_listen_port(443)
            .add_user("test", "password")
            .with_default_padding_scheme();

        assert_eq!(inbound.listen.listen, "::");
        assert_eq!(inbound.listen.listen_port, Some(443));
        assert!(inbound.padding_scheme.is_some());
    }

    #[test]
    fn test_serialize() {
        let inbound = AnyTlsInbound::new("anytls-in")
            .with_listen("::")
            .with_listen_port(443)
            .add_user("sekai", "8JCsPssfgS8tiRwiMlhARg==");

        let json = serde_json::to_string_pretty(&inbound).unwrap();
        assert!(json.contains("\"type\": \"anytls\""));
        assert!(json.contains("\"tag\": \"anytls-in\""));
        assert!(json.contains("\"listen\": \"::\""));
        assert!(json.contains("\"listen_port\": 443"));
        assert!(json.contains("\"name\": \"sekai\""));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "type": "anytls",
            "tag": "anytls-in",
            "listen": "::",
            "listen_port": 443,
            "users": [
                {
                    "name": "sekai",
                    "password": "8JCsPssfgS8tiRwiMlhARg=="
                }
            ],
            "padding_scheme": ["stop=8", "0=30-30"],
            "tls": {
                "enabled": true,
                "server_name": "example.com"
            }
        }"#;

        let inbound: AnyTlsInbound = serde_json::from_str(json).unwrap();
        assert_eq!(inbound.inbound_type, "anytls");
        assert_eq!(inbound.tag, "anytls-in");
        assert_eq!(inbound.listen.listen, "::");
        assert_eq!(inbound.listen.listen_port, Some(443));
        assert_eq!(inbound.users.len(), 1);
        assert_eq!(inbound.users[0].name, "sekai");
        assert!(inbound.padding_scheme.is_some());
        assert!(inbound.tls.is_some());
    }

    #[test]
    fn test_default_padding_scheme() {
        let scheme = AnyTlsInbound::default_padding_scheme();
        assert_eq!(scheme.len(), 9);
        assert_eq!(scheme[0], "stop=8");
    }
}
