use std::net::IpAddr;

use crate::singboxconfig::inbound::{CongestionControl, TuicInbound};
use crate::singboxconfig::shared::{AcmeConfig, InboundTlsConfig};
use crate::singboxconfig::types::{Duration, TuicUser};

// 从tools 模块导入通用功能
use super::tools::{
    PublicIpError, TlsMode, generate_password, generate_sslip_domain, generate_uuid, get_public_ip,
};

//============================================================================
// TUIC 用户配置
//============================================================================

/// TUIC 用户配置
/// 如果只提供 name，则自动生成 uuid 和 password
#[derive(Debug, Clone)]
pub struct TuicUserConfig {
    /// 用户名（可选）
    pub name: Option<String>,
    /// 用户 UUID（可选，不提供则自动生成）
    pub uuid: Option<String>,
    /// 用户密码（可选）
    pub password: Option<String>,
}

impl TuicUserConfig {
    /// 创建新用户配置（自动生成 UUID 和密码）
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            uuid: None,
            password: None,
        }
    }

    /// 创建带UUID 的用户配置
    pub fn with_uuid(name: impl Into<String>, uuid: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            uuid: Some(uuid.into()),
            password: None,
        }
    }

    /// 创建带完整凭证的用户配置
    pub fn with_credentials(
        name: impl Into<String>,
        uuid: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            name: Some(name.into()),
            uuid: Some(uuid.into()),
            password: Some(password.into()),
        }
    }

    /// 仅使用 UUID 创建用户配置
    pub fn uuid_only(uuid: impl Into<String>) -> Self {
        Self {
            name: None,
            uuid: Some(uuid.into()),
            password: None,
        }
    }

    /// 获取 UUID（如果未设置则生成）
    pub fn get_or_generate_uuid(&self) -> String {
        self.uuid.clone().unwrap_or_else(generate_uuid)
    }

    /// 获取密码（如果未设置则生成）
    pub fn get_or_generate_password(&self) -> Option<String> {
        if self.password.is_some() {
            self.password.clone()
        } else {
            Some(generate_password())
        }
    }
}

//============================================================================
// 自动化 TUIC 配置生成器
//============================================================================

/// 自动化 TUIC 配置
#[derive(Debug, Clone)]
pub struct AutoTuicConfig {
    /// 监听端口（默认 443）
    pub port: Option<u16>,
    /// 监听地址（默认 "::"）
    pub listen: Option<String>,
    /// 服务器公网 IP（用于生成 sslip.io 域名）
    pub public_ip: Option<IpAddr>,
    /// 用户列表（如果为空，自动生成一个用户）
    pub users: Vec<TuicUserConfig>,
    /// TLS 配置模式
    pub tls_mode: TlsMode,
    /// 入站标签（默认 "tuic-in"）
    pub tag: Option<String>,
    /// QUIC 拥塞控制算法
    pub congestion_control: Option<CongestionControl>,
    /// 认证超时时间
    pub auth_timeout: Option<Duration>,
    /// 启用 0-RTT 握手（不推荐）
    pub zero_rtt_handshake: Option<bool>,
    /// 心跳间隔
    pub heartbeat: Option<Duration>,
}

impl Default for AutoTuicConfig {
    fn default() -> Self {
        Self {
            port: None,
            listen: None,
            public_ip: None,
            users: Vec::new(),
            tls_mode: TlsMode::default(),
            tag: None,
            congestion_control: None,
            auth_timeout: None,
            zero_rtt_handshake: None,
            heartbeat: None,
        }
    }
}

/// 自动化 TUIC 配置构建器
#[derive(Debug, Default)]
pub struct AutoTuicBuilder {
    config: AutoTuicConfig,
}

impl AutoTuicBuilder {
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
    pub fn auto_detect_ip(mut self) -> Result<Self, PublicIpError> {
        self.config.public_ip = Some(get_public_ip()?);
        Ok(self)
    }

    /// 添加用户（自动生成 UUID 和密码）
    pub fn add_user(mut self, name: impl Into<String>) -> Self {
        self.config.users.push(TuicUserConfig::new(name));
        self
    }

    /// 添加用户（指定 UUID）
    pub fn add_user_with_uuid(mut self, name: impl Into<String>, uuid: impl Into<String>) -> Self {
        self.config
            .users
            .push(TuicUserConfig::with_uuid(name, uuid));
        self
    }

    /// 添加用户（完整凭证）
    pub fn add_user_with_credentials(
        mut self,
        name: impl Into<String>,
        uuid: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.config
            .users
            .push(TuicUserConfig::with_credentials(name, uuid, password));
        self
    }

    /// 设置入站标签
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.config.tag = Some(tag.into());
        self
    }

    /// 设置拥塞控制算法
    pub fn congestion_control(mut self, cc: CongestionControl) -> Self {
        self.config.congestion_control = Some(cc);
        self
    }

    /// 使用 Cubic 拥塞控制
    pub fn cubic(mut self) -> Self {
        self.config.congestion_control = Some(CongestionControl::Cubic);
        self
    }

    /// 使用 BBR 拥塞控制
    pub fn bbr(mut self) -> Self {
        self.config.congestion_control = Some(CongestionControl::Bbr);
        self
    }

    /// 使用 New Reno 拥塞控制
    pub fn new_reno(mut self) -> Self {
        self.config.congestion_control = Some(CongestionControl::NewReno);
        self
    }

    /// 设置认证超时时间
    pub fn auth_timeout(mut self, timeout: Duration) -> Self {
        self.config.auth_timeout = Some(timeout);
        self
    }

    /// 设置认证超时时间（秒）
    pub fn auth_timeout_secs(mut self, secs: u64) -> Self {
        self.config.auth_timeout = Some(Duration::from_secs(secs));
        self
    }

    /// 启用/禁用 0-RTT 握手
    /// 注意: 强烈建议禁用，因为它容易受到重放攻击
    pub fn zero_rtt_handshake(mut self, enabled: bool) -> Self {
        self.config.zero_rtt_handshake = Some(enabled);
        self
    }

    /// 设置心跳间隔
    pub fn heartbeat(mut self, interval: Duration) -> Self {
        self.config.heartbeat = Some(interval);
        self
    }

    /// 设置心跳间隔（秒）
    pub fn heartbeat_secs(mut self, secs: u64) -> Self {
        self.config.heartbeat = Some(Duration::from_secs(secs));
        self
    }

    /// 使用 ACME 自动证书（sslip.io）
    pub fn acme(mut self) -> Self {
        self.config.tls_mode = TlsMode::Acme {
            domain: None,
            email: None,
        };
        self
    }

    /// 使用 ACME 自动证书（指定域名）
    pub fn acme_with_domain(mut self, domain: impl Into<String>) -> Self {
        self.config.tls_mode = TlsMode::Acme {
            domain: Some(domain.into()),
            email: None,
        };
        self
    }

    /// 使用 ACME 自动证书（指定域名和邮箱）
    pub fn acme_with_domain_and_email(
        mut self,
        domain: impl Into<String>,
        email: impl Into<String>,
    ) -> Self {
        self.config.tls_mode = TlsMode::Acme {
            domain: Some(domain.into()),
            email: Some(email.into()),
        };
        self
    }

    /// 使用自定义证书
    pub fn custom_cert(
        mut self,
        certificate_path: impl Into<String>,
        key_path: impl Into<String>,
    ) -> Self {
        self.config.tls_mode = TlsMode::Custom {
            certificate_path: certificate_path.into(),
            key_path: key_path.into(),
            server_name: None,
        };
        self
    }

    /// 使用自定义证书（带服务器名称）
    pub fn custom_cert_with_server_name(
        mut self,
        certificate_path: impl Into<String>,
        key_path: impl Into<String>,
        server_name: impl Into<String>,
    ) -> Self {
        self.config.tls_mode = TlsMode::Custom {
            certificate_path: certificate_path.into(),
            key_path: key_path.into(),
            server_name: Some(server_name.into()),
        };
        self
    }

    /// 构建配置
    pub fn build(self) -> Result<AutoTuicResult, AutoTuicError> {
        self.config.generate()
    }
}

//============================================================================
// 生成结果
//============================================================================

/// 自动化 TUIC 配置生成结果
#[derive(Debug, Clone)]
pub struct AutoTuicResult {
    /// 生成的入站配置
    pub inbound: TuicInbound,
    /// 生成的用户列表（包含实际使用的 UUID 和密码）
    pub users: Vec<TuicUser>,
    /// 使用的域名（如果有）
    pub domain: Option<String>,
    /// 连接信息
    pub connection_info: TuicConnectionInfo,
}

/// TUIC 连接信息
#[derive(Debug, Clone)]
pub struct TuicConnectionInfo {
    /// 服务器地址
    pub server: String,
    /// 服务器端口
    pub port: u16,
    /// 服务器名称（SNI）
    pub server_name: Option<String>,
    ///拥塞控制算法
    pub congestion_control: CongestionControl,
}

//============================================================================
// 错误类型
//============================================================================

/// 自动化 TUIC 配置错误
#[derive(Debug, Clone)]
pub enum AutoTuicError {
    /// 缺少必要配置
    MissingConfig(String),
    /// 配置冲突
    ConfigConflict(String),
    /// 无效配置
    InvalidConfig(String),
}

impl std::fmt::Display for AutoTuicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoTuicError::MissingConfig(msg) => write!(f, "缺少配置: {}", msg),
            AutoTuicError::ConfigConflict(msg) => write!(f, "配置冲突: {}", msg),
            AutoTuicError::InvalidConfig(msg) => write!(f, "无效配置: {}", msg),
        }
    }
}

impl std::error::Error for AutoTuicError {}

//============================================================================
// 配置生成实现
//============================================================================

impl AutoTuicConfig {
    /// 生成配置
    pub fn generate(&self) -> Result<AutoTuicResult, AutoTuicError> {
        // 确定端口
        let port = self.port.unwrap_or(443);

        // 确定监听地址
        let listen = self.listen.clone().unwrap_or_else(|| "::".to_string());

        // 确定标签
        let tag = self.tag.clone().unwrap_or_else(|| "tuic-in".to_string());

        // 生成用户
        let users = self.generate_users();

        // 生成 TLS 配置和域名
        let (tls_config, domain) = self.generate_tls_config()?;

        // 确定拥塞控制算法
        let congestion_control = self
            .congestion_control
            .clone()
            .unwrap_or(CongestionControl::Cubic);

        // 构建入站配置
        let mut inbound = TuicInbound::new(&tag)
            .with_listen(&listen)
            .with_listen_port(port)
            .with_tls(tls_config)
            .with_congestion_control(congestion_control.clone());

        // 添加用户
        for user in &users {
            inbound = inbound.add_user(user.clone());
        }

        // 添加认证超时
        if let Some(timeout) = &self.auth_timeout {
            inbound = inbound.with_auth_timeout(timeout.clone());
        }

        // 添加 0-RTT 握手设置
        if let Some(zero_rtt) = self.zero_rtt_handshake {
            inbound = inbound.with_zero_rtt_handshake(zero_rtt);
        }

        // 添加心跳间隔
        if let Some(heartbeat) = &self.heartbeat {
            inbound = inbound.with_heartbeat(heartbeat.clone());
        }

        // 生成连接信息
        let server = if let Some(ip) = &self.public_ip {
            ip.to_string()
        } else if let Some(ref d) = domain {
            d.clone()
        } else {
            listen.clone()
        };

        let connection_info = TuicConnectionInfo {
            server,
            port,
            server_name: domain.clone(),
            congestion_control,
        };

        Ok(AutoTuicResult {
            inbound,
            users,
            domain,
            connection_info,
        })
    }

    /// 生成用户列表
    fn generate_users(&self) -> Vec<TuicUser> {
        if self.users.is_empty() {
            // 如果没有用户，生成一个默认用户
            vec![TuicUser::with_credentials(
                "default",
                generate_uuid(),
                generate_password(),
            )]
        } else {
            self.users
                .iter()
                .map(|u| {
                    let uuid = u.get_or_generate_uuid();
                    let password = u.get_or_generate_password();
                    let mut user = TuicUser::new(&uuid);
                    if let Some(ref name) = u.name {
                        user = user.with_name(name);
                    }
                    if let Some(pwd) = password {
                        user = user.with_password(pwd);
                    }
                    user
                })
                .collect()
        }
    }

    /// 生成 TLS 配置
    fn generate_tls_config(&self) -> Result<(InboundTlsConfig, Option<String>), AutoTuicError> {
        match &self.tls_mode {
            TlsMode::Acme { domain, email } => {
                let actual_domain = if let Some(d) = domain {
                    d.clone()
                } else if let Some(ip) = &self.public_ip {
                    generate_sslip_domain(ip)
                } else {
                    return Err(AutoTuicError::MissingConfig(
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

                Ok((tls, Some(actual_domain)))
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

                Ok((tls, server_name.clone()))
            }
            TlsMode::Disabled => Err(AutoTuicError::InvalidConfig(
                "TUIC 必须启用 TLS".to_string(),
            )),
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
    fn test_tuic_user_config_new() {
        let user = TuicUserConfig::new("test_user");
        assert_eq!(user.name, Some("test_user".to_string()));
        assert!(user.uuid.is_none());
        assert!(user.password.is_none());
    }

    #[test]
    fn test_tuic_user_config_with_uuid() {
        let user = TuicUserConfig::with_uuid("test_user", "059032A9-7D40-4A96-9BB1-36823D848068");
        assert_eq!(user.name, Some("test_user".to_string()));
        assert_eq!(
            user.uuid,
            Some("059032A9-7D40-4A96-9BB1-36823D848068".to_string())
        );
        assert!(user.password.is_none());
    }

    #[test]
    fn test_tuic_user_config_with_credentials() {
        let user = TuicUserConfig::with_credentials(
            "test_user",
            "059032A9-7D40-4A96-9BB1-36823D848068",
            "hello",
        );
        assert_eq!(user.name, Some("test_user".to_string()));
        assert_eq!(
            user.uuid,
            Some("059032A9-7D40-4A96-9BB1-36823D848068".to_string())
        );
        assert_eq!(user.password, Some("hello".to_string()));
    }

    #[test]
    fn test_tuic_user_config_get_or_generate() {
        let user = TuicUserConfig::new("test");
        let uuid = user.get_or_generate_uuid();
        assert!(!uuid.is_empty());
        assert!(uuid.contains('-')); // UUID format

        let password = user.get_or_generate_password();
        assert!(password.is_some());
        assert!(!password.unwrap().is_empty());
    }

    #[test]
    fn test_builder_default() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let result = AutoTuicBuilder::new()
            .public_ip(ip)
            .port(443)
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.inbound.inbound_type, "tuic");
        assert_eq!(result.inbound.tag, "tuic-in");
        assert_eq!(result.inbound.listen.listen_port, Some(443));
        assert_eq!(result.users.len(), 1);
        assert_eq!(result.connection_info.port, 443);
        assert!(result.domain.is_some());
    }

    #[test]
    fn test_builder_with_congestion_control() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let result = AutoTuicBuilder::new()
            .public_ip(ip)
            .bbr()
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(
            result.inbound.congestion_control,
            Some(CongestionControl::Bbr)
        );
        assert_eq!(
            result.connection_info.congestion_control,
            CongestionControl::Bbr
        );
    }

    #[test]
    fn test_builder_with_zero_rtt() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let result = AutoTuicBuilder::new()
            .public_ip(ip)
            .zero_rtt_handshake(false)
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.inbound.zero_rtt_handshake, Some(false));
    }

    #[test]
    fn test_builder_with_heartbeat() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let result = AutoTuicBuilder::new()
            .public_ip(ip)
            .heartbeat_secs(15)
            .add_user("user1")
            .build()
            .unwrap();

        assert!(result.inbound.heartbeat.is_some());
    }

    #[test]
    fn test_builder_with_auth_timeout() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let result = AutoTuicBuilder::new()
            .public_ip(ip)
            .auth_timeout_secs(5)
            .add_user("user1")
            .build()
            .unwrap();

        assert!(result.inbound.auth_timeout.is_some());
    }

    #[test]
    fn test_builder_custom_domain() {
        let result = AutoTuicBuilder::new()
            .acme_with_domain("example.com")
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.domain, Some("example.com".to_string()));
    }

    #[test]
    fn test_builder_multiple_users() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let result = AutoTuicBuilder::new()
            .public_ip(ip)
            .add_user("user1")
            .add_user("user2")
            .add_user_with_uuid("user3", "059032A9-7D40-4A96-9BB1-36823D848068")
            .build()
            .unwrap();

        assert_eq!(result.users.len(), 3);
        assert_eq!(result.users[2].uuid, "059032A9-7D40-4A96-9BB1-36823D848068");
    }

    #[test]
    fn test_builder_no_users_generates_default() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let result = AutoTuicBuilder::new().public_ip(ip).build().unwrap();

        assert_eq!(result.users.len(), 1);
        assert_eq!(result.users[0].name, Some("default".to_string()));
    }

    #[test]
    fn test_tls_disabled_error() {
        let result = AutoTuicBuilder::new().add_user("user1").build();

        // Should fail because no domain or IP provided for ACME
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_result() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let result = AutoTuicBuilder::new()
            .public_ip(ip)
            .port(443)
            .add_user_with_credentials("sekai", "059032A9-7D40-4A96-9BB1-36823D848068", "hello")
            .cubic()
            .build()
            .unwrap();

        let json = serde_json::to_string_pretty(&result.inbound).unwrap();
        assert!(json.contains("\"type\": \"tuic\""));
        assert!(json.contains("\"congestion_control\": \"cubic\""));
        assert!(json.contains("059032A9-7D40-4A96-9BB1-36823D848068"));
    }

    #[test]
    fn test_custom_cert() {
        let result = AutoTuicBuilder::new()
            .custom_cert_with_server_name("/path/to/cert.pem", "/path/to/key.pem", "example.com")
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.domain, Some("example.com".to_string()));
        assert_eq!(
            result.inbound.tls.certificate_path,
            Some("/path/to/cert.pem".to_string())
        );
        assert_eq!(
            result.inbound.tls.key_path,
            Some("/path/to/key.pem".to_string())
        );
    }
}
