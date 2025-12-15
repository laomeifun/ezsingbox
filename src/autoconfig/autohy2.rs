use std::net::IpAddr;

use crate::singboxconfig::inbound::{Hysteria2Inbound};
use crate::singboxconfig::shared::{AcmeConfig, InboundTlsConfig};
use crate::singboxconfig::types::UserWithPassword;

// 从tools 模块导入通用功能
use super::tools::{
    PublicIpError, TlsMode, UserConfig, generate_password, generate_sslip_domain, get_public_ip,
};

//============================================================================
// 自动化Hysteria2 配置生成器
//============================================================================

/// 自动化 Hysteria2 配置
#[derive(Debug, Clone)]
pub struct AutoHysteria2Config {
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
    /// 入站标签（默认 "hy2-in"）
    pub tag: Option<String>,
    /// 上行带宽限制（Mbps）
    pub up_mbps: Option<u32>,
    /// 下行带宽限制（Mbps）
    pub down_mbps: Option<u32>,
    /// 混淆密码（如果设置则启用 salamander 混淆）
    pub obfs_password: Option<String>,
    /// 伪装 URL
    pub masquerade_url: Option<String>,
    /// 忽略客户端带宽设置
    pub ignore_client_bandwidth: Option<bool>,
}

impl Default for AutoHysteria2Config {
    fn default() -> Self {
        Self {
            port: None,
            listen: None,
            public_ip: None,
            users: Vec::new(),
            tls_mode: TlsMode::default(),
            tag: None,
            up_mbps: None,
            down_mbps: None,
            obfs_password: None,
            masquerade_url: None,
            ignore_client_bandwidth: None,
        }
    }
}

/// 自动化 Hysteria2 配置构建器
#[derive(Debug, Default)]
pub struct AutoHysteria2Builder {
    config: AutoHysteria2Config,
}

impl AutoHysteria2Builder {
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

    /// 设置带宽限制
    pub fn bandwidth(mut self, up_mbps: u32, down_mbps: u32) -> Self {
        self.config.up_mbps = Some(up_mbps);
        self.config.down_mbps = Some(down_mbps);
        self
    }

    /// 设置上行带宽限制
    pub fn up_mbps(mut self, mbps: u32) -> Self {
        self.config.up_mbps = Some(mbps);
        self
    }

    /// 设置下行带宽限制
    pub fn down_mbps(mut self, mbps: u32) -> Self {
        self.config.down_mbps = Some(mbps);
        self
    }

    /// 启用混淆（自动生成密码）
    pub fn with_obfs(mut self) -> Self {
        self.config.obfs_password = Some(generate_password());
        self
    }

    /// 启用混淆（指定密码）
    pub fn with_obfs_password(mut self, password: impl Into<String>) -> Self {
        self.config.obfs_password = Some(password.into());
        self
    }

    /// 设置伪装 URL
    pub fn with_masquerade(mut self, url: impl Into<String>) -> Self {
        self.config.masquerade_url = Some(url.into());
        self
    }

    ///忽略客户端带宽设置
    pub fn ignore_client_bandwidth(mut self, ignore: bool) -> Self {
        self.config.ignore_client_bandwidth = Some(ignore);
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

    /// 构建配置
    pub fn build(self) -> Result<AutoHysteria2Result, AutoHysteria2Error> {
        self.config.generate()
    }
}

/// 生成结果
#[derive(Debug, Clone)]
pub struct AutoHysteria2Result {
    /// 生成的入站配置
    pub inbound: Hysteria2Inbound,
    /// 生成的用户信息（包含密码）
    pub users: Vec<UserWithPassword>,
    /// 使用的域名
    pub domain: Option<String>,
    /// 混淆密码（如果启用）
    pub obfs_password: Option<String>,
    /// 连接信息摘要
    pub connection_info: Hysteria2ConnectionInfo,
}

/// 连接信息
#[derive(Debug, Clone)]
pub struct Hysteria2ConnectionInfo {
    /// 服务器地址
    pub server: String,
    /// 服务器端口
    pub port: u16,
    /// 服务器名称（SNI）
    pub server_name: Option<String>,
    /// 上行带宽
    pub up_mbps: Option<u32>,
    /// 下行带宽
    pub down_mbps: Option<u32>,
    /// 是否启用混淆
    pub obfs_enabled: bool,
}

/// 错误类型
#[derive(Debug, Clone)]
pub enum AutoHysteria2Error {
    /// 缺少必要配置
    MissingConfig(String),
    /// 配置冲突
    ConfigConflict(String),
    /// 无效配置
    InvalidConfig(String),
}

impl std::fmt::Display for AutoHysteria2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoHysteria2Error::MissingConfig(msg) => write!(f, "缺少必要配置: {}", msg),
            AutoHysteria2Error::ConfigConflict(msg) => write!(f, "配置冲突: {}", msg),
            AutoHysteria2Error::InvalidConfig(msg) => write!(f, "无效配置: {}", msg),
        }
    }
}

impl std::error::Error for AutoHysteria2Error {}

impl AutoHysteria2Config {
    /// 生成配置
    pub fn generate(&self) -> Result<AutoHysteria2Result, AutoHysteria2Error> {
        // 确定端口
        let port = self.port.unwrap_or(443);

        // 确定监听地址
        let listen = self.listen.clone().unwrap_or_else(|| "::".to_string());

        // 确定标签
        let tag = self.tag.clone().unwrap_or_else(|| "hy2-in".to_string());

        // 生成用户
        let users = self.generate_users();

        // 生成 TLS 配置和域名
        let (tls_config, domain) = self.generate_tls_config()?;

        // 构建入站配置
        let mut inbound = Hysteria2Inbound::new(&tag)
            .with_listen(&listen)
            .with_listen_port(port)
            .with_tls(tls_config);

        // 添加用户
        for user in &users {
            inbound = inbound.add_user(&user.name, &user.password);
        }

        // 添加带宽限制
        if let (Some(up), Some(down)) = (self.up_mbps, self.down_mbps) {
            inbound = inbound.with_bandwidth(up, down);
        }

        // 添加混淆
        let obfs_password = self.obfs_password.clone();
        if let Some(ref password) = obfs_password {
            inbound = inbound.with_obfs(password);
        }

        // 添加伪装
        if let Some(ref url) = self.masquerade_url {
            inbound = inbound.with_masquerade_url(url);
        }

        // 忽略客户端带宽
        if let Some(ignore) = self.ignore_client_bandwidth {
            inbound = inbound.with_ignore_client_bandwidth(ignore);
        }

        // 生成连接信息
        let server = if let Some(ip) = &self.public_ip {
            ip.to_string()
        } else if let Some(ref d) = domain {
            d.clone()
        } else {
            listen.clone()
        };

        let connection_info = Hysteria2ConnectionInfo {
            server,
            port,
            server_name: domain.clone(),
            up_mbps: self.up_mbps,
            down_mbps: self.down_mbps,
            obfs_enabled: obfs_password.is_some(),
        };

        Ok(AutoHysteria2Result {
            inbound,
            users,
            domain,
            obfs_password,
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
    ) -> Result<(InboundTlsConfig, Option<String>), AutoHysteria2Error> {
        match &self.tls_mode {
            TlsMode::Acme { domain, email } => {
                let actual_domain = if let Some(d) = domain {
                    d.clone()
                } else if let Some(ip) = &self.public_ip {
                    generate_sslip_domain(ip)
                } else {
                    return Err(AutoHysteria2Error::MissingConfig(
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
            TlsMode::Disabled => Err(AutoHysteria2Error::InvalidConfig(
                "Hysteria2 必须启用 TLS".to_string(),
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
    fn test_builder_default() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoHysteria2Builder::new()
            .public_ip(ip)
            .add_user("test_user")
            .build()
            .unwrap();

        assert_eq!(result.inbound.inbound_type, "hysteria2");
        assert_eq!(result.users.len(), 1);
        assert_eq!(result.users[0].name, "test_user");
        assert!(result.domain.is_some());
        assert_eq!(result.connection_info.port, 443);
    }

    #[test]
    fn test_builder_with_bandwidth() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoHysteria2Builder::new()
            .public_ip(ip)
            .bandwidth(100, 200)
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.inbound.up_mbps, Some(100));
        assert_eq!(result.inbound.down_mbps, Some(200));
        assert_eq!(result.connection_info.up_mbps, Some(100));
        assert_eq!(result.connection_info.down_mbps, Some(200));
    }

    #[test]
    fn test_builder_with_obfs() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoHysteria2Builder::new()
            .public_ip(ip)
            .with_obfs_password("my_obfs_password")
            .add_user("user1")
            .build()
            .unwrap();

        assert!(result.inbound.obfs.is_some());
        assert_eq!(result.obfs_password, Some("my_obfs_password".to_string()));
        assert!(result.connection_info.obfs_enabled);
    }

    #[test]
    fn test_builder_with_masquerade() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoHysteria2Builder::new()
            .public_ip(ip)
            .with_masquerade("https://www.bing.com")
            .add_user("user1")
            .build()
            .unwrap();

        assert!(result.inbound.masquerade.is_some());
    }

    #[test]
    fn test_builder_custom_domain() {
        let result = AutoHysteria2Builder::new()
            .acme_with_domain("example.com")
            .add_user("user1")
            .build()
            .unwrap();

        assert_eq!(result.domain, Some("example.com".to_string()));
    }

    #[test]
    fn test_builder_multiple_users() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoHysteria2Builder::new()
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
        let result = AutoHysteria2Builder::new().public_ip(ip).build().unwrap();

        assert_eq!(result.users.len(), 1);
        assert_eq!(result.users[0].name, "default");
    }

    #[test]
    fn test_tls_disabled_error() {
        let mut config = AutoHysteria2Config::default();
        config.tls_mode = TlsMode::disabled();
        config.public_ip = Some("203.0.113.1".parse().unwrap());

        let result = config.generate();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_result() {
        let ip: IpAddr = "203.0.113.1".parse().unwrap();
        let result = AutoHysteria2Builder::new()
            .public_ip(ip)
            .port(443)
            .bandwidth(100, 100)
            .with_obfs_password("cry_me_a_r1ver")
            .add_user("tobyxdd")
            .build()
            .unwrap();

        let json = serde_json::to_string_pretty(&result.inbound).unwrap();
        assert!(json.contains("\"type\": \"hysteria2\""));
        assert!(json.contains("\"listen_port\": 443"));
        assert!(json.contains("\"up_mbps\": 100"));
        assert!(json.contains("\"salamander\""));
    }
}
