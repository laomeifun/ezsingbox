use std::net::IpAddr;

use crate::singboxconfig::inbound::{VlessFlow, VlessInbound, VlessUser};
use crate::singboxconfig::shared::{
    AcmeConfig, InboundTlsConfig, MultiplexInbound, V2RayTransport,
};

//从 tools 模块导入通用功能
use super::tools::{PublicIpError, TlsMode, generate_sslip_domain, generate_uuid, get_public_ip};

//============================================================================
// VLESS 用户配置
//============================================================================

/// VLESS 用户配置
/// 如果只提供 name，则自动生成 UUID
#[derive(Debug, Clone)]
pub struct VlessUserConfig {
    /// 用户名
    pub name: String,
    /// 用户 UUID（可选，不提供则自动生成）
    pub uuid: Option<String>,
    /// VLESS 子协议（可选）
    pub flow: Option<VlessFlow>,
}

impl VlessUserConfig {
    /// 创建新用户配置（自动生成UUID）
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uuid: None,
            flow: None,
        }
    }

    /// 创建带 UUID 的用户配置
    pub fn with_uuid(name: impl Into<String>, uuid: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uuid: Some(uuid.into()),
            flow: None,
        }
    }

    /// 创建带 flow 的用户配置
    pub fn with_flow(name: impl Into<String>, flow: VlessFlow) -> Self {
        Self {
            name: name.into(),
            uuid: None,
            flow: Some(flow),
        }
    }

    /// 创建完整的用户配置
    pub fn full(name: impl Into<String>, uuid: impl Into<String>, flow: VlessFlow) -> Self {
        Self {
            name: name.into(),
            uuid: Some(uuid.into()),
            flow: Some(flow),
        }
    }

    /// 获取 UUID（如果未设置则生成）
    pub fn get_or_generate_uuid(&self) -> String {
        self.uuid.clone().unwrap_or_else(generate_uuid)
    }

    /// 设置 flow
    pub fn set_flow(mut self, flow: VlessFlow) -> Self {
        self.flow = Some(flow);
        self
    }

    /// 启用 XTLS Vision
    pub fn xtls_vision(mut self) -> Self {
        self.flow = Some(VlessFlow::XtlsRprxVision);
        self
    }
}

//============================================================================
// 自动化 VLESS 配置生成器
//============================================================================

/// 自动化 VLESS 配置
#[derive(Debug, Clone)]
pub struct AutoVlessConfig {
    /// 监听端口（默认 443）
    pub port: Option<u16>,
    /// 监听地址（默认 "::"）
    pub listen: Option<String>,
    /// 服务器公网 IP（用于生成 sslip.io 域名）
    pub public_ip: Option<IpAddr>,
    /// 用户列表（如果为空，自动生成一个用户）
    pub users: Vec<VlessUserConfig>,
    /// TLS 配置模式
    pub tls_mode: TlsMode,
    /// 入站标签（默认 "vless-in"）
    pub tag: Option<String>,
    /// 是否启用多路复用
    pub multiplex: Option<MultiplexInbound>,
    /// V2Ray 传输配置
    pub transport: Option<V2RayTransport>,
    /// 默认启用 XTLS Vision flow
    pub default_xtls_vision: bool,
}

impl Default for AutoVlessConfig {
    fn default() -> Self {
        Self {
            port: None,
            listen: None,
            public_ip: None,
            users: Vec::new(),
            tls_mode: TlsMode::default(),
            tag: None,
            multiplex: None,
            transport: None,
            default_xtls_vision: false,
        }
    }
}

/// 自动化 VLESS 配置构建器
#[derive(Debug, Default)]
pub struct AutoVlessBuilder {
    config: AutoVlessConfig,
}

impl AutoVlessBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置监听端口
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = Some(port);
        self
    }

    /// 设置监听地址
    pub fn listen(mut self, listen: impl Into<String>) -> Self {
        self.config.listen = Some(listen.into());
        self
    }

    /// 设置公网 IP（用于 sslip.io）
    pub fn public_ip(mut self, ip: IpAddr) -> Self {
        self.config.public_ip = Some(ip);
        self
    }

    /// 从字符串解析并设置公网 IP
    pub fn public_ip_str(mut self, ip: &str) -> Result<Self, std::net::AddrParseError> {
        self.config.public_ip = Some(ip.parse()?);
        Ok(self)
    }

    /// 自动获取公网 IP
    /// 通过调用外部服务获取当前服务器的公网 IP
    pub fn auto_detect_ip(mut self) -> Result<Self, PublicIpError> {
        self.config.public_ip = Some(get_public_ip()?);
        Ok(self)
    }

    /// 添加用户（自动生成 UUID）
    pub fn add_user(mut self, name: impl Into<String>) -> Self {
        self.config.users.push(VlessUserConfig::new(name));
        self
    }

    /// 添加用户（指定 UUID）
    pub fn add_user_with_uuid(mut self, name: impl Into<String>, uuid: impl Into<String>) -> Self {
        self.config
            .users
            .push(VlessUserConfig::with_uuid(name, uuid));
        self
    }

    /// 添加用户（带XTLS Vision flow）
    pub fn add_user_with_xtls_vision(mut self, name: impl Into<String>) -> Self {
        self.config
            .users
            .push(VlessUserConfig::with_flow(name, VlessFlow::XtlsRprxVision));
        self
    }

    /// 添加用户（指定 UUID 和 flow）
    pub fn add_user_full(
        mut self,
        name: impl Into<String>,
        uuid: impl Into<String>,
        flow: VlessFlow,
    ) -> Self {
        self.config
            .users
            .push(VlessUserConfig::full(name, uuid, flow));
        self
    }

    /// 添加自定义用户配置
    pub fn add_user_config(mut self, user: VlessUserConfig) -> Self {
        self.config.users.push(user);
        self
    }

    /// 设置入站标签
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.config.tag = Some(tag.into());
        self
    }

    /// 使用 ACME 自动证书（默认）
    pub fn acme(mut self) -> Self {
        self.config.tls_mode = TlsMode::acme();
        self
    }

    /// 使用 ACME 自动证书，指定域名
    pub fn acme_with_domain(mut self, domain: impl Into<String>) -> Self {
        self.config.tls_mode = TlsMode::acme_with_domain(domain);
        self
    }

    /// 使用 ACME 自动证书，指定域名和邮箱
    pub fn acme_with_domain_and_email(
        mut self,
        domain: impl Into<String>,
        email: impl Into<String>,
    ) -> Self {
        self.config.tls_mode = TlsMode::acme_with_domain_and_email(domain, email);
        self
    }

    /// 使用自定义证书
    pub fn custom_cert(
        mut self,
        certificate_path: impl Into<String>,
        key_path: impl Into<String>,
    ) -> Self {
        self.config.tls_mode = TlsMode::custom(certificate_path, key_path);
        self
    }

    /// 使用自定义证书，指定服务器名称
    pub fn custom_cert_with_server_name(
        mut self,
        certificate_path: impl Into<String>,
        key_path: impl Into<String>,
        server_name: impl Into<String>,
    ) -> Self {
        self.config.tls_mode =
            TlsMode::custom_with_server_name(certificate_path, key_path, server_name);
        self
    }

    /// 禁用 TLS（不推荐，仅用于测试）
    pub fn disable_tls(mut self) -> Self {
        self.config.tls_mode = TlsMode::Disabled;
        self
    }

    /// 启用多路复用
    pub fn with_multiplex(mut self) -> Self {
        self.config.multiplex = Some(MultiplexInbound::default());
        self
    }

    /// 启用多路复用（自定义配置）
    pub fn with_multiplex_config(mut self, multiplex: MultiplexInbound) -> Self {
        self.config.multiplex = Some(multiplex);
        self
    }

    /// 设置 V2Ray 传输配置
    pub fn with_transport(mut self, transport: V2RayTransport) -> Self {
        self.config.transport = Some(transport);
        self
    }

    /// 为所有用户默认启用 XTLS Vision
    pub fn default_xtls_vision(mut self) -> Self {
        self.config.default_xtls_vision = true;
        self
    }

    /// 构建配置
    pub fn build(self) -> Result<AutoVlessResult, AutoVlessError> {
        self.config.generate()
    }
}

//============================================================================
// 生成结果
//============================================================================

/// 生成结果
#[derive(Debug, Clone)]
pub struct AutoVlessResult {
    /// 生成的入站配置
    pub inbound: VlessInbound,
    /// 生成的用户信息（包含 UUID）
    pub users: Vec<VlessUser>,
    /// 使用的域名
    pub domain: Option<String>,
    /// 连接信息摘要
    pub connection_info: VlessConnectionInfo,
}

/// 连接信息
#[derive(Debug, Clone)]
pub struct VlessConnectionInfo {
    /// 服务器地址
    pub server: String,
    /// 服务器端口
    pub port: u16,
    /// 服务器名称（SNI）
    pub server_name: Option<String>,
    /// 是否启用 TLS
    pub tls_enabled: bool,
    /// 传输类型
    pub transport_type: Option<String>,
}

//============================================================================
// 错误类型
//============================================================================

/// 错误类型
#[derive(Debug, Clone)]
pub enum AutoVlessError {
    /// 缺少必要配置
    MissingConfig(String),
    /// 配置冲突
    ConfigConflict(String),
    /// 无效配置
    InvalidConfig(String),
}

impl std::fmt::Display for AutoVlessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoVlessError::MissingConfig(msg) => write!(f, "缺少必要配置: {}", msg),
            AutoVlessError::ConfigConflict(msg) => write!(f, "配置冲突: {}", msg),
            AutoVlessError::InvalidConfig(msg) => write!(f, "无效配置: {}", msg),
        }
    }
}

impl std::error::Error for AutoVlessError {}

//============================================================================
// 配置生成实现
//============================================================================

impl AutoVlessConfig {
    /// 生成配置
    pub fn generate(&self) -> Result<AutoVlessResult, AutoVlessError> {
        // 确定端口
        let port = self.port.unwrap_or(443);

        // 确定监听地址
        let listen = self.listen.clone().unwrap_or_else(|| "::".to_string());

        // 确定标签
        let tag = self.tag.clone().unwrap_or_else(|| "vless-in".to_string());

        // 生成用户
        let users = self.generate_users();

        // 生成 TLS 配置和域名
        let (tls_config, domain, tls_enabled) = self.generate_tls_config()?;

        // 构建入站配置
        let mut inbound = VlessInbound::new(&tag)
            .with_listen(&listen)
            .with_listen_port(port);

        // 添加用户
        for user in &users {
            inbound = inbound.add_user(user.clone());
        }

        // 添加 TLS 配置
        if let Some(tls) = tls_config {
            inbound = inbound.with_tls(tls);
        }

        // 添加多路复用配置
        if let Some(ref multiplex) = self.multiplex {
            inbound = inbound.with_multiplex(multiplex.clone());
        }

        // 添加传输配置
        if let Some(ref transport) = self.transport {
            inbound = inbound.with_transport(transport.clone());
        }

        // 生成连接信息
        let server = if let Some(ip) = &self.public_ip {
            ip.to_string()
        } else if let Some(ref d) = domain {
            d.clone()
        } else {
            listen.clone()
        };

        let transport_type = self.transport.as_ref().map(|t| match t {
            V2RayTransport::Http(_) => "http".to_string(),
            V2RayTransport::Ws(_) => "ws".to_string(),
            V2RayTransport::Quic(_) => "quic".to_string(),
            V2RayTransport::Grpc(_) => "grpc".to_string(),
            V2RayTransport::HttpUpgrade(_) => "httpupgrade".to_string(),
        });

        let connection_info = VlessConnectionInfo {
            server,
            port,
            server_name: domain.clone(),
            tls_enabled,
            transport_type,
        };

        Ok(AutoVlessResult {
            inbound,
            users,
            domain,
            connection_info,
        })
    }

    /// 生成用户列表
    fn generate_users(&self) -> Vec<VlessUser> {
        if self.users.is_empty() {
            // 如果没有用户，生成一个默认用户
            let mut user = VlessUser::new("default", generate_uuid());
            if self.default_xtls_vision {
                user = user.with_xtls_vision();
            }
            vec![user]
        } else {
            self.users
                .iter()
                .map(|u| {
                    let uuid = u.get_or_generate_uuid();
                    let mut user = VlessUser::new(&u.name, uuid);

                    // 设置 flow
                    if let Some(ref flow) = u.flow {
                        user = user.set_flow(flow.clone());
                    } else if self.default_xtls_vision {
                        user = user.with_xtls_vision();
                    }

                    user
                })
                .collect()
        }
    }

    /// 生成 TLS 配置
    fn generate_tls_config(
        &self,
    ) -> Result<(Option<InboundTlsConfig>, Option<String>, bool), AutoVlessError> {
        match &self.tls_mode {
            TlsMode::Acme { domain, email } => {
                let actual_domain = if let Some(d) = domain {
                    d.clone()
                } else if let Some(ip) = &self.public_ip {
                    generate_sslip_domain(ip)
                } else {
                    return Err(AutoVlessError::MissingConfig(
                        "使用 ACME 时需要提供域名或公网 IP".to_string(),
                    ));
                };

                let acme = AcmeConfig {
                    domain: Some(vec![actual_domain.clone()]),
                    email: email.clone(),
                    ..Default::default()
                };

                let tls = InboundTlsConfig {
                    enabled: Some(true),
                    server_name: Some(actual_domain.clone()),
                    acme: Some(acme),
                    ..Default::default()
                };

                Ok((Some(tls), Some(actual_domain), true))
            }
            TlsMode::Custom {
                certificate_path,
                key_path,
                server_name,
            } => {
                let tls = InboundTlsConfig {
                    enabled: Some(true),
                    server_name: server_name.clone(),
                    certificate_path: Some(certificate_path.clone()),
                    key_path: Some(key_path.clone()),
                    ..Default::default()
                };

                Ok((Some(tls), server_name.clone(), true))
            }
            TlsMode::Disabled => Ok((None, None, false)),
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
    fn test_vless_user_config_new() {
        let user = VlessUserConfig::new("test_user");
        assert_eq!(user.name, "test_user");
        assert!(user.uuid.is_none());
        assert!(user.flow.is_none());
    }

    #[test]
    fn test_vless_user_config_with_uuid() {
        let user = VlessUserConfig::with_uuid("test_user", "bf000d23-0752-40b4-affe-68f7707a9661");
        assert_eq!(user.name, "test_user");
        assert_eq!(
            user.uuid,
            Some("bf000d23-0752-40b4-affe-68f7707a9661".to_string())
        );
    }

    #[test]
    fn test_vless_user_config_with_flow() {
        let user = VlessUserConfig::with_flow("test_user", VlessFlow::XtlsRprxVision);
        assert_eq!(user.flow, Some(VlessFlow::XtlsRprxVision));
    }

    #[test]
    fn test_vless_user_config_xtls_vision() {
        let user = VlessUserConfig::new("test_user").xtls_vision();
        assert_eq!(user.flow, Some(VlessFlow::XtlsRprxVision));
    }

    #[test]
    fn test_vless_user_config_get_or_generate_uuid() {
        let user1 = VlessUserConfig::new("user1");
        let uuid1 = user1.get_or_generate_uuid();
        assert!(!uuid1.is_empty());
        assert!(uuid1.contains('-')); // UUID format

        let user2 = VlessUserConfig::with_uuid("user2", "fixed-uuid");
        let uuid2 = user2.get_or_generate_uuid();
        assert_eq!(uuid2, "fixed-uuid");
    }

    #[test]
    fn test_builder_default() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoVlessBuilder::new()
            .public_ip(ip)
            .add_user("test_user")
            .build()
            .unwrap();

        assert_eq!(result.inbound.inbound_type, "vless");
        assert_eq!(result.users.len(), 1);
        assert_eq!(result.users[0].name, "test_user");
        assert!(result.domain.is_some());
        assert_eq!(result.connection_info.port, 443);
        assert!(result.connection_info.tls_enabled);
    }

    #[test]
    fn test_builder_custom_port() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoVlessBuilder::new()
            .public_ip(ip)
            .port(8443)
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.connection_info.port, 8443);
    }

    #[test]
    fn test_builder_custom_domain() {
        let result = AutoVlessBuilder::new()
            .acme_with_domain("example.com")
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.domain, Some("example.com".to_string()));
    }

    #[test]
    fn test_builder_custom_cert() {
        let result = AutoVlessBuilder::new()
            .custom_cert_with_server_name("/path/to/cert.pem", "/path/to/key.pem", "example.com")
            .add_user("user1")
            .build()
            .unwrap();

        assert!(result.inbound.tls.is_some());
    }

    #[test]
    fn test_builder_multiple_users() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoVlessBuilder::new()
            .public_ip(ip)
            .add_user("user1")
            .add_user_with_uuid("user2", "custom-uuid-1234")
            .add_user_with_xtls_vision("user3")
            .build()
            .unwrap();

        assert_eq!(result.users.len(), 3);
        assert_eq!(result.users[1].uuid, "custom-uuid-1234");
        assert_eq!(result.users[2].flow, Some(VlessFlow::XtlsRprxVision));
    }

    #[test]
    fn test_builder_no_users_generates_default() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoVlessBuilder::new().public_ip(ip).build().unwrap();

        assert_eq!(result.users.len(), 1);
        assert_eq!(result.users[0].name, "default");
    }

    #[test]
    fn test_builder_default_xtls_vision() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoVlessBuilder::new()
            .public_ip(ip)
            .default_xtls_vision()
            .add_user("user1")
            .add_user("user2")
            .build()
            .unwrap();

        //所有用户都应该有 XTLS Vision flow
        for user in &result.users {
            assert_eq!(user.flow, Some(VlessFlow::XtlsRprxVision));
        }
    }

    #[test]
    fn test_builder_with_multiplex() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoVlessBuilder::new()
            .public_ip(ip)
            .add_user("user1")
            .with_multiplex()
            .build()
            .unwrap();

        assert!(result.inbound.multiplex.is_some());
    }

    #[test]
    fn test_builder_disable_tls() {
        let result = AutoVlessBuilder::new()
            .disable_tls()
            .add_user("user1")
            .build()
            .unwrap();

        assert!(result.inbound.tls.is_none());
        assert!(!result.connection_info.tls_enabled);
    }

    #[test]
    fn test_serialize_result() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoVlessBuilder::new()
            .public_ip(ip)
            .port(443)
            .add_user("sekai")
            .build()
            .unwrap();

        let json = serde_json::to_string_pretty(&result.inbound).unwrap();
        assert!(json.contains("\"type\": \"vless\""));
        assert!(json.contains("\"listen_port\": 443"));
    }

    #[test]
    fn test_builder_full_config() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoVlessBuilder::new()
            .public_ip(ip)
            .port(443)
            .listen("::")
            .tag("my-vless-in")
            .add_user_full(
                "sekai",
                "bf000d23-0752-40b4-affe-68f7707a9661",
                VlessFlow::XtlsRprxVision,
            )
            .with_multiplex()
            .build()
            .unwrap();

        assert_eq!(result.inbound.tag, "my-vless-in");
        assert_eq!(result.users[0].name, "sekai");
        assert_eq!(result.users[0].uuid, "bf000d23-0752-40b4-affe-68f7707a9661");
        assert_eq!(result.users[0].flow, Some(VlessFlow::XtlsRprxVision));
        assert!(result.inbound.multiplex.is_some());
    }

    #[test]
    fn test_error_missing_config() {
        // 没有提供公网 IP 或域名时应该报错
        let result = AutoVlessBuilder::new().add_user("user1").build();

        assert!(result.is_err());
        if let Err(AutoVlessError::MissingConfig(msg)) = result {
            assert!(msg.contains("ACME"));
        } else {
            panic!("Expected MissingConfig error");
        }
    }
}
