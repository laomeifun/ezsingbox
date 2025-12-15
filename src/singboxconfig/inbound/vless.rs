use serde::{Deserialize, Serialize};

use crate::singboxconfig::shared::{
    InboundTlsConfig, ListenFields, MultiplexInbound, V2RayTransport,
};

//============================================================================
// VLESS入站配置（服务端）
//============================================================================

/// VLESS 入站配置（服务端）
/// 文档: https://sing-box.sagernet.org/configuration/inbound/vless/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VlessInbound {
    /// 入站类型，固定为 "vless"
    #[serde(rename = "type")]
    pub inbound_type: String,

    /// 入站标签
    pub tag: String,

    /// 监听字段
    #[serde(flatten)]
    pub listen: ListenFields,

    /// VLESS 用户列表（必填）
    pub users: Vec<VlessUser>,

    /// TLS 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<InboundTlsConfig>,

    /// 多路复用配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<MultiplexInbound>,

    /// V2Ray 传输配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<V2RayTransport>,
}

//============================================================================
// VLESS 用户
//============================================================================

/// VLESS 用户
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct VlessUser {
    /// 用户名
    pub name: String,

    /// 用户 UUID（必填）
    pub uuid: String,

    /// VLESS 子协议
    /// 可用值: xtls-rprx-vision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<VlessFlow>,
}

//============================================================================
// VLESS Flow 子协议
//============================================================================

/// VLESS 子协议
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum VlessFlow {
    /// XTLS Vision 流控
    XtlsRprxVision,
}

//============================================================================
// VlessInbound 实现
//============================================================================

impl VlessInbound {
    /// 创建新的 VLESS 入站配置
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            inbound_type: "vless".to_string(),
            tag: tag.into(),
            listen: ListenFields::default(),
            users: Vec::new(),
            tls: None,
            multiplex: None,
            transport: None,
        }
    }

    /// 添加用户
    pub fn add_user(mut self, user: VlessUser) -> Self {
        self.users.push(user);
        self
    }

    /// 添加简单用户（仅名称和 UUID）
    pub fn add_simple_user(mut self, name: impl Into<String>, uuid: impl Into<String>) -> Self {
        self.users.push(VlessUser::new(name, uuid));
        self
    }

    /// 添加带flow 的用户
    pub fn add_user_with_flow(
        mut self,
        name: impl Into<String>,
        uuid: impl Into<String>,
        flow: VlessFlow,
    ) -> Self {
        self.users.push(VlessUser::with_flow(name, uuid, flow));
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

    /// 设置监听字段
    pub fn with_listen_fields(mut self, listen: ListenFields) -> Self {
        self.listen = listen;
        self
    }

    /// 设置 TLS 配置
    pub fn with_tls(mut self, tls: InboundTlsConfig) -> Self {
        self.tls = Some(tls);
        self
    }

    /// 设置多路复用配置
    pub fn with_multiplex(mut self, multiplex: MultiplexInbound) -> Self {
        self.multiplex = Some(multiplex);
        self
    }

    /// 设置 V2Ray 传输配置
    pub fn with_transport(mut self, transport: V2RayTransport) -> Self {
        self.transport = Some(transport);
        self
    }
}

impl Default for VlessInbound {
    fn default() -> Self {
        Self::new("")
    }
}

//============================================================================
// VlessUser 实现
//============================================================================

impl VlessUser {
    /// 创建新的 VLESS 用户
    pub fn new(name: impl Into<String>, uuid: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uuid: uuid.into(),
            flow: None,
        }
    }

    /// 创建带 flow 的 VLESS 用户
    pub fn with_flow(name: impl Into<String>, uuid: impl Into<String>, flow: VlessFlow) -> Self {
        Self {
            name: name.into(),
            uuid: uuid.into(),
            flow: Some(flow),
        }
    }

    /// 设置 flow
    pub fn set_flow(mut self, flow: VlessFlow) -> Self {
        self.flow = Some(flow);
        self
    }

    /// 启用 XTLS Vision
    pub fn with_xtls_vision(mut self) -> Self {
        self.flow = Some(VlessFlow::XtlsRprxVision);
        self
    }
}

impl Default for VlessUser {
    fn default() -> Self {
        Self {
            name: String::new(),
            uuid: String::new(),
            flow: None,
        }
    }
}

//============================================================================
// 单元测试
//============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inbound = VlessInbound::new("vless-in");
        assert_eq!(inbound.inbound_type, "vless");
        assert_eq!(inbound.tag, "vless-in");
        assert!(inbound.users.is_empty());
    }

    #[test]
    fn test_add_simple_user() {
        let inbound = VlessInbound::new("vless-in")
            .add_simple_user("sekai", "bf000d23-0752-40b4-affe-68f7707a9661");

        assert_eq!(inbound.users.len(), 1);
        assert_eq!(inbound.users[0].name, "sekai");
        assert_eq!(
            inbound.users[0].uuid,
            "bf000d23-0752-40b4-affe-68f7707a9661"
        );
        assert!(inbound.users[0].flow.is_none());
    }

    #[test]
    fn test_add_user_with_flow() {
        let inbound = VlessInbound::new("vless-in").add_user_with_flow(
            "sekai",
            "bf000d23-0752-40b4-affe-68f7707a9661",
            VlessFlow::XtlsRprxVision,
        );

        assert_eq!(inbound.users.len(), 1);
        assert_eq!(inbound.users[0].flow, Some(VlessFlow::XtlsRprxVision));
    }

    #[test]
    fn test_vless_user_new() {
        let user = VlessUser::new("sekai", "bf000d23-0752-40b4-affe-68f7707a9661");
        assert_eq!(user.name, "sekai");
        assert_eq!(user.uuid, "bf000d23-0752-40b4-affe-68f7707a9661");
        assert!(user.flow.is_none());
    }

    #[test]
    fn test_vless_user_with_xtls_vision() {
        let user =
            VlessUser::new("sekai", "bf000d23-0752-40b4-affe-68f7707a9661").with_xtls_vision();
        assert_eq!(user.flow, Some(VlessFlow::XtlsRprxVision));
    }

    #[test]
    fn test_vless_flow_serialize() {
        let flow = VlessFlow::XtlsRprxVision;
        let json = serde_json::to_string(&flow).unwrap();
        assert_eq!(json, "\"xtls-rprx-vision\"");
    }

    #[test]
    fn test_vless_flow_deserialize() {
        let flow: VlessFlow = serde_json::from_str("\"xtls-rprx-vision\"").unwrap();
        assert_eq!(flow, VlessFlow::XtlsRprxVision);
    }

    #[test]
    fn test_serialize() {
        let inbound = VlessInbound::new("vless-in")
            .with_listen("::")
            .with_listen_port(443)
            .add_simple_user("sekai", "bf000d23-0752-40b4-affe-68f7707a9661");

        let json = serde_json::to_string_pretty(&inbound).unwrap();
        assert!(json.contains("\"type\": \"vless\""));
        assert!(json.contains("\"tag\": \"vless-in\""));
        assert!(json.contains("\"uuid\": \"bf000d23-0752-40b4-affe-68f7707a9661\""));
    }

    #[test]
    fn test_serialize_with_flow() {
        let inbound = VlessInbound::new("vless-in")
            .with_listen("::")
            .with_listen_port(443)
            .add_user_with_flow(
                "sekai",
                "bf000d23-0752-40b4-affe-68f7707a9661",
                VlessFlow::XtlsRprxVision,
            );

        let json = serde_json::to_string_pretty(&inbound).unwrap();
        assert!(json.contains("\"flow\": \"xtls-rprx-vision\""));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "type": "vless",
            "tag": "vless-in",
            "listen": "::",
            "listen_port": 443,
            "users": [
                {
                    "name": "sekai",
                    "uuid": "bf000d23-0752-40b4-affe-68f7707a9661",
                    "flow": "xtls-rprx-vision"
                }
            ]
        }"#;

        let inbound: VlessInbound = serde_json::from_str(json).unwrap();
        assert_eq!(inbound.inbound_type, "vless");
        assert_eq!(inbound.tag, "vless-in");
        assert_eq!(inbound.users.len(), 1);
        assert_eq!(inbound.users[0].name, "sekai");
        assert_eq!(
            inbound.users[0].uuid,
            "bf000d23-0752-40b4-affe-68f7707a9661"
        );
        assert_eq!(inbound.users[0].flow, Some(VlessFlow::XtlsRprxVision));
    }

    #[test]
    fn test_deserialize_without_flow() {
        let json = r#"{
            "type": "vless",
            "tag": "vless-in",
            "listen": "::",
            "listen_port": 443,
            "users": [
                {
                    "name": "sekai",
                    "uuid": "bf000d23-0752-40b4-affe-68f7707a9661"
                }
            ]
        }"#;

        let inbound: VlessInbound = serde_json::from_str(json).unwrap();
        assert_eq!(inbound.users[0].flow, None);
    }

    #[test]
    fn test_full_config() {
        let inbound = VlessInbound::new("vless-in")
            .with_listen("::")
            .with_listen_port(443)
            .add_user_with_flow(
                "sekai",
                "bf000d23-0752-40b4-affe-68f7707a9661",
                VlessFlow::XtlsRprxVision,
            )
            .with_tls(InboundTlsConfig::default())
            .with_multiplex(MultiplexInbound::default());

        assert!(inbound.tls.is_some());
        assert!(inbound.multiplex.is_some());
    }
}
