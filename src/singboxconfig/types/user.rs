use serde::{Deserialize, Serialize};

//============================================================================
// 通用用户类型
//============================================================================

//============================================================================
// VLESS Flow 类型
// ============================================================================

/// VLESS 子协议流类型
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum VlessFlow {
    /// XTLS Vision 流控
    XtlsRprxVision,
}

/// 带密码的用户
/// 用于 AnyTLS、Trojan、Shadowsocks 等协议
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserWithPassword {
    /// 用户名
    pub name: String,

    /// 用户密码
    pub password: String,
}

impl UserWithPassword {
    /// 创建新用户
    pub fn new(name: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            password: password.into(),
        }
    }
}

impl Default for UserWithPassword {
    fn default() -> Self {
        Self {
            name: String::new(),
            password: String::new(),
        }
    }
}

// ============================================================================
// VMess 用户类型
// ============================================================================

/// VMess 用户
/// VMess 协议使用 UUID 进行身份验证
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct VMessUser {
    /// 用户名
    pub name: String,

    /// 用户 UUID
    pub uuid: String,

    /// Alter ID
    /// 0 =禁用旧版协议
    /// > 0 = 启用旧版协议（不推荐使用 alterId > 1）
    #[serde(rename = "alterId")]
    #[serde(default)]
    pub alter_id: u32,
}

impl VMessUser {
    /// 创建新的 VMess 用户
    pub fn new(name: impl Into<String>, uuid: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uuid: uuid.into(),
            alter_id: 0,
        }
    }

    /// 创建带Alter ID 的 VMess 用户
    pub fn with_alter_id(name: impl Into<String>, uuid: impl Into<String>, alter_id: u32) -> Self {
        Self {
            name: name.into(),
            uuid: uuid.into(),
            alter_id,
        }
    }
}

impl Default for VMessUser {
    fn default() -> Self {
        Self {
            name: String::new(),
            uuid: String::new(),
            alter_id: 0,
        }
    }
}

// ============================================================================
// VLESS 用户类型
// ============================================================================

/// VLESS 用户
/// VLESS 协议使用 UUID 进行身份验证
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

impl VlessUser {
    /// 创建新的 VLESS 用户
    pub fn new(name: impl Into<String>, uuid: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uuid: uuid.into(),
            flow: None,
        }
    }

    /// 创建带flow 的 VLESS 用户
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

    /// 设置 XTLS Vision flow
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

// ============================================================================
// TUIC 用户类型
// ============================================================================

/// TUIC 用户
/// TUIC 协议使用 UUID 和密码进行身份验证
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TuicUser {
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 用户 UUID（必填）
    pub uuid: String,

    /// 用户密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl TuicUser {
    /// 创建新的 TUIC 用户（仅 UUID）
    pub fn new(uuid: impl Into<String>) -> Self {
        Self {
            name: None,
            uuid: uuid.into(),
            password: None,
        }
    }

    /// 创建带名称和密码的 TUIC 用户
    pub fn with_credentials(
        name: impl Into<String>,
        uuid: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            name: Some(name.into()),
            uuid: uuid.into(),
            password: Some(password.into()),
        }
    }

    /// 设置用户名
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置密码
    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }
}

impl Default for TuicUser {
    fn default() -> Self {
        Self {
            name: None,
            uuid: String::new(),
            password: None,
        }
    }
}

// ============================================================================
// Shadowsocks 中继目标
// ============================================================================

/// Shadowsocks 中继目标
/// 用于 Shadowsocks 中继模式
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShadowsocksDestination {
    /// 目标名称
    pub name: String,

    /// 目标服务器地址
    pub server: String,

    /// 目标服务器端口
    pub server_port: u16,

    /// 目标密码
    pub password: String,
}

impl ShadowsocksDestination {
    /// 创建新的中继目标
    pub fn new(
        name: impl Into<String>,
        server: impl Into<String>,
        server_port: u16,
        password: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            server: server.into(),
            server_port,
            password: password.into(),
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
    fn test_user_with_password_new() {
        let user = UserWithPassword::new("sekai", "8JCsPssfgS8tiRwiMlhARg==");
        assert_eq!(user.name, "sekai");
        assert_eq!(user.password, "8JCsPssfgS8tiRwiMlhARg==");
    }

    #[test]
    fn test_user_with_password_serialize() {
        let user = UserWithPassword::new("sekai", "password123");
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"name\":\"sekai\""));
        assert!(json.contains("\"password\":\"password123\""));
    }

    #[test]
    fn test_user_with_password_deserialize() {
        let json = r#"{"name":"sekai","password":"8JCsPssfgS8tiRwiMlhARg=="}"#;
        let user: UserWithPassword = serde_json::from_str(json).unwrap();
        assert_eq!(user.name, "sekai");
        assert_eq!(user.password, "8JCsPssfgS8tiRwiMlhARg==");
    }

    #[test]
    fn test_vmess_user_new() {
        let user = VMessUser::new("sekai", "bf000d23-0752-40b4-affe-68f7707a9661");
        assert_eq!(user.name, "sekai");
        assert_eq!(user.uuid, "bf000d23-0752-40b4-affe-68f7707a9661");
        assert_eq!(user.alter_id, 0);
    }

    #[test]
    fn test_vmess_user_with_alter_id() {
        let user = VMessUser::with_alter_id("sekai", "bf000d23-0752-40b4-affe-68f7707a9661", 1);
        assert_eq!(user.alter_id, 1);
    }

    #[test]
    fn test_vmess_user_serialize() {
        let user = VMessUser::new("sekai", "bf000d23-0752-40b4-affe-68f7707a9661");
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"name\":\"sekai\""));
        assert!(json.contains("\"uuid\":\"bf000d23-0752-40b4-affe-68f7707a9661\""));
        assert!(json.contains("\"alterId\":0"));
    }

    #[test]
    fn test_vmess_user_deserialize() {
        let json = r#"{"name":"sekai","uuid":"bf000d23-0752-40b4-affe-68f7707a9661","alterId":0}"#;
        let user: VMessUser = serde_json::from_str(json).unwrap();
        assert_eq!(user.name, "sekai");
        assert_eq!(user.uuid, "bf000d23-0752-40b4-affe-68f7707a9661");
        assert_eq!(user.alter_id, 0);
    }

    #[test]
    fn test_shadowsocks_destination_new() {
        let dest = ShadowsocksDestination::new("test", "example.com", 8080, "password");
        assert_eq!(dest.name, "test");
        assert_eq!(dest.server, "example.com");
        assert_eq!(dest.server_port, 8080);
        assert_eq!(dest.password, "password");
    }

    #[test]
    fn test_shadowsocks_destination_serialize() {
        let dest = ShadowsocksDestination::new("test", "example.com", 8080, "password");
        let json = serde_json::to_string(&dest).unwrap();
        assert!(json.contains("\"name\":\"test\""));
        assert!(json.contains("\"server\":\"example.com\""));
        assert!(json.contains("\"server_port\":8080"));
        assert!(json.contains("\"password\":\"password\""));
    }

    #[test]
    fn test_vless_user_new() {
        let user = VlessUser::new("sekai", "bf000d23-0752-40b4-affe-68f7707a9661");
        assert_eq!(user.name, "sekai");
        assert_eq!(user.uuid, "bf000d23-0752-40b4-affe-68f7707a9661");
        assert!(user.flow.is_none());
    }

    #[test]
    fn test_vless_user_with_flow() {
        let user = VlessUser::with_flow(
            "sekai",
            "bf000d23-0752-40b4-affe-68f7707a9661",
            VlessFlow::XtlsRprxVision,
        );
        assert_eq!(user.flow, Some(VlessFlow::XtlsRprxVision));
    }

    #[test]
    fn test_vless_user_with_xtls_vision() {
        let user =
            VlessUser::new("sekai", "bf000d23-0752-40b4-affe-68f7707a9661").with_xtls_vision();
        assert_eq!(user.flow, Some(VlessFlow::XtlsRprxVision));
    }

    #[test]
    fn test_vless_user_serialize() {
        let user = VlessUser::new("sekai", "bf000d23-0752-40b4-affe-68f7707a9661");
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"name\":\"sekai\""));
        assert!(json.contains("\"uuid\":\"bf000d23-0752-40b4-affe-68f7707a9661\""));
        // flow为 None 时不应该序列化
        assert!(!json.contains("flow"));
    }

    #[test]
    fn test_vless_user_serialize_with_flow() {
        let user =
            VlessUser::new("sekai", "bf000d23-0752-40b4-affe-68f7707a9661").with_xtls_vision();
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"flow\":\"xtls-rprx-vision\""));
    }

    #[test]
    fn test_vless_user_deserialize() {
        let json = r#"{"name":"sekai","uuid":"bf000d23-0752-40b4-affe-68f7707a9661"}"#;
        let user: VlessUser = serde_json::from_str(json).unwrap();
        assert_eq!(user.name, "sekai");
        assert_eq!(user.uuid, "bf000d23-0752-40b4-affe-68f7707a9661");
        assert!(user.flow.is_none());
    }

    #[test]
    fn test_vless_user_deserialize_with_flow() {
        let json = r#"{"name":"sekai","uuid":"bf000d23-0752-40b4-affe-68f7707a9661","flow":"xtls-rprx-vision"}"#;
        let user: VlessUser = serde_json::from_str(json).unwrap();
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
}
