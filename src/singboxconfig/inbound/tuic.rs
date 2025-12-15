use serde::{Deserialize, Serialize};

use crate::singboxconfig::shared::{InboundTlsConfig, ListenFields};
use crate::singboxconfig::types::{Duration, TuicUser};

//============================================================================
// QUIC 拥塞控制算法
//============================================================================

/// QUIC 拥塞控制算法
/// 文档: https://sing-box.sagernet.org/configuration/inbound/tuic/
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CongestionControl {
    /// Cubic 拥塞控制算法（默认）
    #[default]
    Cubic,
    /// New Reno 拥塞控制算法
    NewReno,
    /// BBR 拥塞控制算法
    Bbr,
}

//============================================================================
// TUIC 入站配置（服务端）
//============================================================================

/// TUIC 入站配置（服务端）
/// 文档: https://sing-box.sagernet.org/configuration/inbound/tuic/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TuicInbound {
    /// 入站类型，固定为 "tuic"
    #[serde(rename = "type")]
    pub inbound_type: String,

    /// 入站标签
    pub tag: String,

    /// 监听字段
    #[serde(flatten)]
    pub listen: ListenFields,

    /// TUIC 用户列表
    pub users: Vec<TuicUser>,

    /// QUIC 拥塞控制算法
    /// 可选值: cubic, new_reno, bbr
    /// 默认: cubic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub congestion_control: Option<CongestionControl>,

    /// 服务器等待客户端发送认证命令的超时时间
    /// 默认: 3s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_timeout: Option<Duration>,

    /// 启用 0-RTT QUIC 连接握手
    /// 强烈建议禁用，因为它容易受到重放攻击
    /// 默认: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_rtt_handshake: Option<bool>,

    /// 发送心跳包以保持连接活跃的间隔
    /// 默认: 10s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat: Option<Duration>,

    /// TLS 配置（必填）
    pub tls: InboundTlsConfig,
}

impl TuicInbound {
    /// 创建新的 TUIC 入站配置
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            inbound_type: "tuic".to_string(),
            tag: tag.into(),
            listen: ListenFields::default(),
            users: Vec::new(),
            congestion_control: None,
            auth_timeout: None,
            zero_rtt_handshake: None,
            heartbeat: None,
            tls: InboundTlsConfig::default(),
        }
    }

    /// 添加用户
    pub fn add_user(mut self, user: TuicUser) -> Self {
        self.users.push(user);
        self
    }

    /// 添加用户（使用 UUID）
    pub fn add_user_uuid(mut self, uuid: impl Into<String>) -> Self {
        self.users.push(TuicUser::new(uuid));
        self
    }

    /// 添加用户（使用完整凭证）
    pub fn add_user_with_credentials(
        mut self,
        name: impl Into<String>,
        uuid: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.users
            .push(TuicUser::with_credentials(name, uuid, password));
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

    /// 设置拥塞控制算法
    pub fn with_congestion_control(mut self, cc: CongestionControl) -> Self {
        self.congestion_control = Some(cc);
        self
    }

    /// 设置认证超时时间
    pub fn with_auth_timeout(mut self, timeout: Duration) -> Self {
        self.auth_timeout = Some(timeout);
        self
    }

    /// 启用/禁用 0-RTT握手
    /// 注意: 强烈建议禁用，因为它容易受到重放攻击
    pub fn with_zero_rtt_handshake(mut self, enabled: bool) -> Self {
        self.zero_rtt_handshake = Some(enabled);
        self
    }

    /// 设置心跳间隔
    pub fn with_heartbeat(mut self, interval: Duration) -> Self {
        self.heartbeat = Some(interval);
        self
    }

    /// 设置 TLS 配置
    pub fn with_tls(mut self, tls: InboundTlsConfig) -> Self {
        self.tls = tls;
        self
    }

    /// 设置监听字段
    pub fn with_listen_fields(mut self, listen: ListenFields) -> Self {
        self.listen = listen;
        self
    }
}

impl Default for TuicInbound {
    fn default() -> Self {
        Self::new("")
    }
}

//============================================================================
// 单元测试
//============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_congestion_control_serialize() {
        assert_eq!(
            serde_json::to_string(&CongestionControl::Cubic).unwrap(),
            "\"cubic\""
        );
        assert_eq!(
            serde_json::to_string(&CongestionControl::NewReno).unwrap(),
            "\"new_reno\""
        );
        assert_eq!(
            serde_json::to_string(&CongestionControl::Bbr).unwrap(),
            "\"bbr\""
        );
    }

    #[test]
    fn test_congestion_control_deserialize() {
        assert_eq!(
            serde_json::from_str::<CongestionControl>("\"cubic\"").unwrap(),
            CongestionControl::Cubic
        );
        assert_eq!(
            serde_json::from_str::<CongestionControl>("\"new_reno\"").unwrap(),
            CongestionControl::NewReno
        );
        assert_eq!(
            serde_json::from_str::<CongestionControl>("\"bbr\"").unwrap(),
            CongestionControl::Bbr
        );
    }

    #[test]
    fn test_new() {
        let inbound = TuicInbound::new("tuic-in");
        assert_eq!(inbound.inbound_type, "tuic");
        assert_eq!(inbound.tag, "tuic-in");
        assert!(inbound.users.is_empty());
        assert!(inbound.congestion_control.is_none());
        assert!(inbound.auth_timeout.is_none());
        assert!(inbound.zero_rtt_handshake.is_none());
        assert!(inbound.heartbeat.is_none());
    }

    #[test]
    fn test_add_user() {
        let inbound = TuicInbound::new("tuic-in").add_user_with_credentials(
            "sekai",
            "059032A9-7D40-4A96-9BB1-36823D848068",
            "hello",
        );

        assert_eq!(inbound.users.len(), 1);
        assert_eq!(inbound.users[0].name, Some("sekai".to_string()));
        assert_eq!(
            inbound.users[0].uuid,
            "059032A9-7D40-4A96-9BB1-36823D848068"
        );
        assert_eq!(inbound.users[0].password, Some("hello".to_string()));
    }

    #[test]
    fn test_add_user_uuid_only() {
        let inbound =
            TuicInbound::new("tuic-in").add_user_uuid("059032A9-7D40-4A96-9BB1-36823D848068");

        assert_eq!(inbound.users.len(), 1);
        assert!(inbound.users[0].name.is_none());
        assert_eq!(
            inbound.users[0].uuid,
            "059032A9-7D40-4A96-9BB1-36823D848068"
        );
        assert!(inbound.users[0].password.is_none());
    }

    #[test]
    fn test_with_congestion_control() {
        let inbound = TuicInbound::new("tuic-in").with_congestion_control(CongestionControl::Bbr);

        assert_eq!(inbound.congestion_control, Some(CongestionControl::Bbr));
    }

    #[test]
    fn test_with_zero_rtt() {
        let inbound = TuicInbound::new("tuic-in").with_zero_rtt_handshake(false);

        assert_eq!(inbound.zero_rtt_handshake, Some(false));
    }

    #[test]
    fn test_serialize() {
        let inbound = TuicInbound::new("tuic-in")
            .with_listen("::")
            .with_listen_port(443)
            .add_user_with_credentials("sekai", "059032A9-7D40-4A96-9BB1-36823D848068", "hello")
            .with_congestion_control(CongestionControl::Cubic);

        let json = serde_json::to_string_pretty(&inbound).unwrap();
        assert!(json.contains("\"type\": \"tuic\""));
        assert!(json.contains("\"tag\": \"tuic-in\""));
        assert!(json.contains("\"congestion_control\": \"cubic\""));
        assert!(json.contains("\"uuid\": \"059032A9-7D40-4A96-9BB1-36823D848068\""));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "type": "tuic",
            "tag": "tuic-in",
            "listen": "::",
            "listen_port": 443,
            "users": [
                {
                    "name": "sekai",
                    "uuid": "059032A9-7D40-4A96-9BB1-36823D848068",
                    "password": "hello"
                }
            ],
            "congestion_control": "cubic",
            "auth_timeout": "3s",
            "zero_rtt_handshake": false,
            "heartbeat": "10s",
            "tls": {
                "enabled": true
            }
        }"#;

        let inbound: TuicInbound = serde_json::from_str(json).unwrap();
        assert_eq!(inbound.inbound_type, "tuic");
        assert_eq!(inbound.tag, "tuic-in");
        assert_eq!(inbound.users.len(), 1);
        assert_eq!(inbound.users[0].name, Some("sekai".to_string()));
        assert_eq!(
            inbound.users[0].uuid,
            "059032A9-7D40-4A96-9BB1-36823D848068"
        );
        assert_eq!(inbound.users[0].password, Some("hello".to_string()));
        assert_eq!(inbound.congestion_control, Some(CongestionControl::Cubic));
        assert!(inbound.auth_timeout.is_some());
        assert_eq!(inbound.zero_rtt_handshake, Some(false));
        assert!(inbound.heartbeat.is_some());
    }

    #[test]
    fn test_deserialize_minimal() {
        let json = r#"{
            "type": "tuic",
            "tag": "tuic-in",
            "listen": "::",
            "users": [
                {
                    "uuid": "059032A9-7D40-4A96-9BB1-36823D848068"
                }
            ],
            "tls": {
                "enabled": true
            }
        }"#;

        let inbound: TuicInbound = serde_json::from_str(json).unwrap();
        assert_eq!(inbound.inbound_type, "tuic");
        assert_eq!(inbound.users.len(), 1);
        assert!(inbound.users[0].name.is_none());
        assert!(inbound.users[0].password.is_none());
        assert!(inbound.congestion_control.is_none());
    }
}
