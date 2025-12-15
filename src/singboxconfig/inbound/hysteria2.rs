use serde::{Deserialize, Serialize};

use crate::singboxconfig::shared::{InboundTlsConfig, ListenFields};
use crate::singboxconfig::types::UserWithPassword;

//============================================================================
// Hysteria2 入站配置（服务端）
//============================================================================

/// Hysteria2 入站配置（服务端）
/// 文档: https://sing-box.sagernet.org/configuration/inbound/hysteria2/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hysteria2Inbound {
    /// 入站类型，固定为 "hysteria2"
    #[serde(rename = "type")]
    pub inbound_type: String,

    /// 入站标签
    pub tag: String,

    /// 监听字段
    #[serde(flatten)]
    pub listen: ListenFields,

    /// 上行带宽限制（Mbps）
    /// 不设置则不限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_mbps: Option<u32>,

    /// 下行带宽限制（Mbps）
    /// 不设置则不限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_mbps: Option<u32>,

    /// QUIC 流量混淆配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfs: Option<Hysteria2Obfs>,

    /// Hysteria2 用户列表（必填）
    pub users: Vec<UserWithPassword>,

    /// 忽略客户端带宽设置
    /// 当 up_mbps 和 down_mbps 未设置时：命令客户端使用 BBR CC 而不是 Hysteria CC
    /// 当 up_mbps 和 down_mbps 已设置时：拒绝客户端使用 BBR CC
    /// 自sing-box 1.11.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_client_bandwidth: Option<bool>,

    /// TLS 配置（必填）
    pub tls: InboundTlsConfig,

    /// 伪装配置
    /// 认证失败时的HTTP3 服务器行为
    /// 可以是 URL 字符串或配置对象
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Hysteria2Masquerade>,

    /// 启用 Hysteria Brutal CC 的调试信息日志
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brutal_debug: Option<bool>,
}

/// Hysteria2 混淆配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hysteria2Obfs {
    /// 混淆类型，目前只支持 "salamander"
    #[serde(rename = "type")]
    pub obfs_type: String,

    /// 混淆密码
    pub password: String,
}

/// Hysteria2 伪装配置
/// 可以是 URL 字符串或配置对象
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Hysteria2Masquerade {
    /// URL 字符串配置
    /// 例如: "file:///var/www" 或 "http://127.0.0.1:8080"
    Url(String),
    /// 对象配置
    Config(Hysteria2MasqueradeConfig),
}

/// Hysteria2 伪装配置对象
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hysteria2MasqueradeConfig {
    /// 伪装类型
    /// file: 作为文件服务器
    /// proxy: 作为反向代理
    /// string: 返回固定响应
    #[serde(rename = "type")]
    pub masquerade_type: MasqueradeType,

    /// 文件服务器根目录（type = "file" 时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,

    /// 反向代理目标 URL（type = "proxy" 时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// 重写 Host 头到目标 URL（type = "proxy" 时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite_host: Option<bool>,

    /// 固定响应状态码（type = "string" 时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<u16>,

    /// 固定响应头（type = "string" 时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    /// 固定响应内容（type = "string" 时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 伪装类型
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MasqueradeType {
    /// 作为文件服务器
    File,
    /// 作为反向代理
    Proxy,
    /// 返回固定响应
    String,
}

impl Hysteria2Inbound {
    /// 创建新的 Hysteria2 入站配置
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            inbound_type: "hysteria2".to_string(),
            tag: tag.into(),
            listen: ListenFields::default(),
            up_mbps: None,
            down_mbps: None,
            obfs: None,
            users: Vec::new(),
            ignore_client_bandwidth: None,
            tls: InboundTlsConfig::default(),
            masquerade: None,
            brutal_debug: None,
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

    /// 设置带宽限制
    pub fn with_bandwidth(mut self, up_mbps: u32, down_mbps: u32) -> Self {
        self.up_mbps = Some(up_mbps);
        self.down_mbps = Some(down_mbps);
        self
    }

    /// 设置混淆
    pub fn with_obfs(mut self, password: impl Into<String>) -> Self {
        self.obfs = Some(Hysteria2Obfs {
            obfs_type: "salamander".to_string(),
            password: password.into(),
        });
        self
    }

    /// 设置TLS 配置
    pub fn with_tls(mut self, tls: InboundTlsConfig) -> Self {
        self.tls = tls;
        self
    }

    /// 设置 URL伪装
    pub fn with_masquerade_url(mut self, url: impl Into<String>) -> Self {
        self.masquerade = Some(Hysteria2Masquerade::Url(url.into()));
        self
    }

    /// 设置文件服务器伪装
    pub fn with_masquerade_file(mut self, directory: impl Into<String>) -> Self {
        self.masquerade = Some(Hysteria2Masquerade::Config(Hysteria2MasqueradeConfig {
            masquerade_type: MasqueradeType::File,
            directory: Some(directory.into()),
            url: None,
            rewrite_host: None,
            status_code: None,
            headers: None,
            content: None,
        }));
        self
    }

    /// 设置反向代理伪装
    pub fn with_masquerade_proxy(mut self, url: impl Into<String>, rewrite_host: bool) -> Self {
        self.masquerade = Some(Hysteria2Masquerade::Config(Hysteria2MasqueradeConfig {
            masquerade_type: MasqueradeType::Proxy,
            directory: None,
            url: Some(url.into()),
            rewrite_host: Some(rewrite_host),
            status_code: None,
            headers: None,
            content: None,
        }));
        self
    }

    /// 忽略客户端带宽设置
    pub fn with_ignore_client_bandwidth(mut self, ignore: bool) -> Self {
        self.ignore_client_bandwidth = Some(ignore);
        self
    }

    /// 启用调试模式
    pub fn with_brutal_debug(mut self, debug: bool) -> Self {
        self.brutal_debug = Some(debug);
        self
    }

    /// 设置监听字段
    pub fn with_listen_fields(mut self, listen: ListenFields) -> Self {
        self.listen = listen;
        self
    }
}

impl Hysteria2Obfs {
    /// 创建 salamander 混淆
    pub fn salamander(password: impl Into<String>) -> Self {
        Self {
            obfs_type: "salamander".to_string(),
            password: password.into(),
        }
    }
}

impl Default for Hysteria2Inbound {
    fn default() -> Self {
        Self::new("")
    }
}

//============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inbound = Hysteria2Inbound::new("hy2-in");
        assert_eq!(inbound.inbound_type, "hysteria2");
        assert_eq!(inbound.tag, "hy2-in");
        assert!(inbound.users.is_empty());
    }

    #[test]
    fn test_add_user() {
        let inbound = Hysteria2Inbound::new("hy2-in")
            .add_user("user1", "password1")
            .add_user("user2", "password2");

        assert_eq!(inbound.users.len(), 2);
        assert_eq!(inbound.users[0].name, "user1");
        assert_eq!(inbound.users[0].password, "password1");
    }

    #[test]
    fn test_with_obfs() {
        let inbound = Hysteria2Inbound::new("hy2-in").with_obfs("my_obfs_password");

        assert!(inbound.obfs.is_some());
        let obfs = inbound.obfs.unwrap();
        assert_eq!(obfs.obfs_type, "salamander");
        assert_eq!(obfs.password, "my_obfs_password");
    }

    #[test]
    fn test_with_bandwidth() {
        let inbound = Hysteria2Inbound::new("hy2-in").with_bandwidth(100, 200);

        assert_eq!(inbound.up_mbps, Some(100));
        assert_eq!(inbound.down_mbps, Some(200));
    }

    #[test]
    fn test_serialize() {
        let inbound = Hysteria2Inbound::new("hy2-in")
            .with_listen("::")
            .with_listen_port(443)
            .add_user("tobyxdd", "goofy_ahh_password")
            .with_obfs("cry_me_a_r1ver");

        let json = serde_json::to_string_pretty(&inbound).unwrap();
        assert!(json.contains("\"type\": \"hysteria2\""));
        assert!(json.contains("\"tag\": \"hy2-in\""));
        assert!(json.contains("\"salamander\""));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "type": "hysteria2",
            "tag": "hy2-in",
            "listen": "::",
            "listen_port": 443,
            "up_mbps": 100,
            "down_mbps": 100,
            "obfs": {
                "type": "salamander",
                "password": "cry_me_a_r1ver"
            },
            "users": [
                {
                    "name": "tobyxdd",
                    "password": "goofy_ahh_password"
                }
            ],
            "tls": {
                "enabled": true
            }
        }"#;

        let inbound: Hysteria2Inbound = serde_json::from_str(json).unwrap();
        assert_eq!(inbound.inbound_type, "hysteria2");
        assert_eq!(inbound.tag, "hy2-in");
        assert_eq!(inbound.up_mbps, Some(100));
        assert_eq!(inbound.users.len(), 1);
        assert!(inbound.obfs.is_some());
    }

    #[test]
    fn test_masquerade_url() {
        let inbound = Hysteria2Inbound::new("hy2-in").with_masquerade_url("https://www.bing.com");

        if let Some(Hysteria2Masquerade::Url(url)) = inbound.masquerade {
            assert_eq!(url, "https://www.bing.com");
        } else {
            panic!("Expected URL masquerade");
        }
    }

    #[test]
    fn test_masquerade_file() {
        let inbound = Hysteria2Inbound::new("hy2-in").with_masquerade_file("/var/www");

        if let Some(Hysteria2Masquerade::Config(config)) = inbound.masquerade {
            assert!(matches!(config.masquerade_type, MasqueradeType::File));
            assert_eq!(config.directory, Some("/var/www".to_string()));
        } else {
            panic!("Expected Config masquerade");
        }
    }
}
