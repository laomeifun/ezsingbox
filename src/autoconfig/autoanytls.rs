use std::net::IpAddr;

use crate::singboxconfig::inbound::AnyTlsInbound;
use crate::singboxconfig::shared::{AcmeConfig, InboundTlsConfig};
use crate::singboxconfig::types::UserWithPassword;

// 从 tools 模块导入通用功能
use super::tools::{
    PublicIpError, TlsMode, UserConfig, generate_password, generate_sslip_domain, get_public_ip,
};

//============================================================================
// 自动化 AnyTLS 配置生成器
//============================================================================

/// 自动化 AnyTLS 配置
#[derive(Debug, Clone)]
pub struct AutoAnyTlsConfig {
    /// 监听端口（默认 443）
    pub port: Option<u16>,
    /// 监听地址（默认 "::"）
    pub listen: Option<String>,
    /// 服务器公网 IP（用于生成 sslip.io 域名）
    pub public_ip: Option<IpAddr>,
    /// 用户列表（如果为空，自动生成一个用户）
    pub users: Vec<UserConfig>,
    /// TLS 配置模式
    pub tls_mode: TlsMode,
    /// 入站标签（默认 "anytls-in"）
    pub tag: Option<String>,
    /// 是否使用默认填充方案
    pub use_default_padding: bool,
}

impl Default for AutoAnyTlsConfig {
    fn default() -> Self {
        Self {
            port: None,
            listen: None,
            public_ip: None,
            users: Vec::new(),
            tls_mode: TlsMode::default(),
            tag: None,
            use_default_padding: true,
        }
    }
}

/// 自动化 AnyTLS 配置构建器
#[derive(Debug, Default)]
pub struct AutoAnyTlsBuilder {
    config: AutoAnyTlsConfig,
}

impl AutoAnyTlsBuilder {
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

    /// 添加用户（自动生成密码）
    pub fn add_user(mut self, name: impl Into<String>) -> Self {
        self.config.users.push(UserConfig::new(name));
        self
    }

    /// 添加用户（指定密码）
    pub fn add_user_with_password(
        mut self,
        name: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.config
            .users
            .push(UserConfig::with_password(name, password));
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

    /// 禁用 TLS（不推荐）
    pub fn disable_tls(mut self) -> Self {
        self.config.tls_mode = TlsMode::disabled();
        self
    }

    /// 禁用默认填充方案
    pub fn no_padding(mut self) -> Self {
        self.config.use_default_padding = false;
        self
    }

    /// 构建配置
    pub fn build(self) -> Result<AutoAnyTlsResult, AutoAnyTlsError> {
        self.config.generate()
    }
}

/// 生成结果
#[derive(Debug, Clone)]
pub struct AutoAnyTlsResult {
    /// 生成的入站配置
    pub inbound: AnyTlsInbound,
    /// 生成的用户信息（包含密码）
    pub users: Vec<UserWithPassword>,
    /// 使用的域名
    pub domain: Option<String>,
    /// 连接信息摘要
    pub connection_info: ConnectionInfo,
}

/// 连接信息
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// 服务器地址
    pub server: String,
    /// 服务器端口
    pub port: u16,
    /// 服务器名称（SNI）
    pub server_name: Option<String>,
}

/// 错误类型
#[derive(Debug, Clone)]
pub enum AutoAnyTlsError {
    /// 缺少必要配置
    MissingConfig(String),
    /// 配置冲突
    ConfigConflict(String),
    /// 无效配置
    InvalidConfig(String),
}

impl std::fmt::Display for AutoAnyTlsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoAnyTlsError::MissingConfig(msg) => write!(f, "缺少必要配置: {}", msg),
            AutoAnyTlsError::ConfigConflict(msg) => write!(f, "配置冲突: {}", msg),
            AutoAnyTlsError::InvalidConfig(msg) => write!(f, "无效配置: {}", msg),
        }
    }
}

impl std::error::Error for AutoAnyTlsError {}

impl AutoAnyTlsConfig {
    /// 生成配置
    pub fn generate(&self) -> Result<AutoAnyTlsResult, AutoAnyTlsError> {
        // 确定端口
        let port = self.port.unwrap_or(443);

        // 确定监听地址
        let listen = self.listen.clone().unwrap_or_else(|| "::".to_string());

        // 确定标签
        let tag = self.tag.clone().unwrap_or_else(|| "anytls-in".to_string());

        // 生成用户
        let users = self.generate_users();

        // 生成 TLS 配置和域名
        let (tls_config, domain) = self.generate_tls_config()?;

        // 构建入站配置
        let mut inbound = AnyTlsInbound::new(&tag)
            .with_listen(&listen)
            .with_listen_port(port);

        // 添加用户
        for user in &users {
            inbound = inbound.add_user(&user.name, &user.password);
        }

        // 添加填充方案
        if self.use_default_padding {
            inbound = inbound.with_default_padding_scheme();
        }

        // 添加 TLS 配置
        if let Some(tls) = tls_config {
            inbound = inbound.with_tls(tls);
        }

        // 生成连接信息
        let server = if let Some(ip) = &self.public_ip {
            ip.to_string()
        } else if let Some(ref d) = domain {
            d.clone()
        } else {
            listen.clone()
        };

        let connection_info = ConnectionInfo {
            server,
            port,
            server_name: domain.clone(),
        };

        Ok(AutoAnyTlsResult {
            inbound,
            users,
            domain,
            connection_info,
        })
    }

    /// 生成用户列表
    fn generate_users(&self) -> Vec<UserWithPassword> {
        if self.users.is_empty() {
            // 如果没有用户，生成一个默认用户
            vec![UserWithPassword::new("default", generate_password())]
        } else {
            self.users
                .iter()
                .map(|u| {
                    let password = u.get_or_generate_password();
                    UserWithPassword::new(&u.name, password)
                })
                .collect()
        }
    }

    /// 生成 TLS 配置
    fn generate_tls_config(
        &self,
    ) -> Result<(Option<InboundTlsConfig>, Option<String>), AutoAnyTlsError> {
        match &self.tls_mode {
            TlsMode::Acme { domain, email } => {
                let actual_domain = if let Some(d) = domain {
                    d.clone()
                } else if let Some(ip) = &self.public_ip {
                    generate_sslip_domain(ip)
                } else {
                    return Err(AutoAnyTlsError::MissingConfig(
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

                Ok((Some(tls), Some(actual_domain)))
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

                Ok((Some(tls), server_name.clone()))
            }
            TlsMode::Disabled => Ok((None, None)),
        }
    }
}

//============================================================================
// 为 AcmeConfig 实现 Default
//============================================================================

impl Default for AcmeConfig {
    fn default() -> Self {
        Self {
            domain: None,
            data_directory: None,
            default_server_name: None,
            email: None,
            provider: None,
            disable_http_challenge: None,
            disable_tls_alpn_challenge: None,
            alternative_http_port: None,
            alternative_tls_port: None,
            external_account: None,
            dns01_challenge: None,
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
    fn test_builder_default() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoAnyTlsBuilder::new()
            .public_ip(ip)
            .add_user("test_user")
            .build()
            .unwrap();

        assert_eq!(result.inbound.inbound_type, "anytls");
        assert_eq!(result.users.len(), 1);
        assert_eq!(result.users[0].name, "test_user");
        assert!(result.domain.is_some());
        assert_eq!(result.connection_info.port, 443);
    }

    #[test]
    fn test_builder_custom_port() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoAnyTlsBuilder::new()
            .public_ip(ip)
            .port(8443)
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.connection_info.port, 8443);
    }

    #[test]
    fn test_builder_custom_domain() {
        let result = AutoAnyTlsBuilder::new()
            .acme_with_domain("example.com")
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.domain, Some("example.com".to_string()));
    }

    #[test]
    fn test_builder_custom_cert() {
        let result = AutoAnyTlsBuilder::new()
            .custom_cert_with_server_name("/path/to/cert.pem", "/path/to/key.pem", "example.com")
            .add_user("user1")
            .build()
            .unwrap();

        assert!(result.inbound.tls.is_some());
    }

    #[test]
    fn test_builder_multiple_users() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoAnyTlsBuilder::new()
            .public_ip(ip)
            .add_user("user1")
            .add_user_with_password("user2", "custom_password")
            .add_user("user3")
            .build()
            .unwrap();

        assert_eq!(result.users.len(), 3);
        assert_eq!(result.users[1].password, "custom_password");
    }

    #[test]
    fn test_builder_no_users_generates_default() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoAnyTlsBuilder::new().public_ip(ip).build().unwrap();

        assert_eq!(result.users.len(), 1);
        assert_eq!(result.users[0].name, "default");
    }

    #[test]
    fn test_serialize_result() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoAnyTlsBuilder::new()
            .public_ip(ip)
            .port(443)
            .add_user("sekai")
            .build()
            .unwrap();

        let json = serde_json::to_string_pretty(&result.inbound).unwrap();
        assert!(json.contains("\"type\": \"anytls\""));
        assert!(json.contains("\"listen_port\": 443"));
    }
}
