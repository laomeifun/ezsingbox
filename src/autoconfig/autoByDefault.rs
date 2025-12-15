//! 全自动化默认配置生成模块
//!
//! 此模块提供完全自动化的配置生成功能，使用最少的配置即可部署服务。
//!
//! # 特性
//! - 自动检测公网IP
//! - 自动生成 sslip.io 域名用于 ACME 证书
//! - 默认端口优先级: 443, 2053, 2083, 2096, 8443, 993, 995
//! - 支持 AnyTLS、Hysteria2、TUIC 三种协议
//! - 自动生成用户凭证

use std::net::IpAddr;

use crate::singboxconfig::inbound::{
    AnyTlsInbound, CongestionControl, Hysteria2Inbound, TuicInbound, VlessFlow, VlessInbound,
    VlessUser,
};
use crate::singboxconfig::shared::{
    AcmeConfig, InboundTlsConfig, RealityHandshake, RealityInboundConfig,
};
use crate::singboxconfig::types::TuicUser;

use super::tools::{
    PublicIpError, generate_hex_string, generate_password, generate_sslip_domain, generate_uuid,
    get_public_ip,
};

//============================================================================
// 默认端口列表
//============================================================================

/// 默认端口优先级列表
/// Cloudflare 支持的 HTTPS 端口
pub const DEFAULT_PORTS: &[u16] = &[443, 2053, 2083, 2096, 8443, 993, 995];

/// 获取默认端口（第一个）
pub fn default_port() -> u16 {
    DEFAULT_PORTS[0]
}

/// 获取备用端口（指定索引）
pub fn fallback_port(index: usize) -> u16 {
    DEFAULT_PORTS.get(index).copied().unwrap_or(443)
}

//============================================================================
// 协议类型
//============================================================================

/// 支持的协议类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    /// AnyTLS 协议
    AnyTls,
    /// Hysteria2 协议
    Hysteria2,
    /// TUIC 协议
    Tuic,
    /// VLESS-Vision-uTLS-REALITY 协议
    VlessReality,
}

impl Protocol {
    /// 获取协议的默认标签
    pub fn default_tag(&self) -> &'static str {
        match self {
            Protocol::AnyTls => "anytls-in",
            Protocol::Hysteria2 => "hy2-in",
            Protocol::Tuic => "tuic-in",
            Protocol::VlessReality => "vless-reality-in",
        }
    }
}

//============================================================================
// 用户信息
//============================================================================

/// 生成的用户信息
#[derive(Debug, Clone)]
pub struct GeneratedUser {
    /// 用户名
    pub name: String,
    /// 密码
    pub password: String,
    /// UUID（仅 TUIC 使用）
    pub uuid: Option<String>,
}

impl GeneratedUser {
    /// 创建新用户（自动生成密码）
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            password: generate_password(),
            uuid: None,
        }
    }

    /// 创建带UUID 的用户（用于 TUIC）
    pub fn with_uuid(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            password: generate_password(),
            uuid: Some(generate_uuid()),
        }
    }

    /// 创建带指定密码的用户
    pub fn with_password(name: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            password: password.into(),
            uuid: None,
        }
    }

    /// 创建带指定凭证的用户（用于 TUIC）
    pub fn with_credentials(
        name: impl Into<String>,
        password: impl Into<String>,
        uuid: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            password: password.into(),
            uuid: Some(uuid.into()),
        }
    }
}

//============================================================================
// 自动配置结果
//============================================================================

/// 自动配置生成结果
#[derive(Debug, Clone)]
pub struct AutoDefaultResult {
    /// 服务器公网 IP
    pub public_ip: IpAddr,
    /// 使用的域名
    pub domain: String,
    /// 使用的端口
    pub port: u16,
    /// 生成的用户列表
    pub users: Vec<GeneratedUser>,
}

/// AnyTLS 自动配置结果
#[derive(Debug)]
pub struct AnyTlsAutoResult {
    /// 基础信息
    pub info: AutoDefaultResult,
    /// 生成的入站配置
    pub inbound: AnyTlsInbound,
}

/// Hysteria2 自动配置结果
#[derive(Debug)]
pub struct Hysteria2AutoResult {
    /// 基础信息
    pub info: AutoDefaultResult,
    /// 生成的入站配置
    pub inbound: Hysteria2Inbound,
    /// 混淆密码（如果启用）
    pub obfs_password: Option<String>,
}

/// TUIC 自动配置结果
#[derive(Debug)]
pub struct TuicAutoResult {
    /// 基础信息
    pub info: AutoDefaultResult,
    /// 生成的入站配置
    pub inbound: TuicInbound,
}

/// VLESS-Vision-uTLS-REALITY 自动配置结果
#[derive(Debug)]
pub struct VlessRealityAutoResult {
    /// 基础信息
    pub info: AutoDefaultResult,
    /// 生成的入站配置
    pub inbound: VlessInbound,
    /// REALITY 私钥（服务端使用）
    pub private_key: String,
    /// REALITY 公钥（客户端使用）
    pub public_key: String,
    /// REALITY 短 ID
    pub short_id: String,
    /// 握手服务器地址
    pub handshake_server: String,
    /// 握手服务器端口
    pub handshake_port: u16,
}

/// REALITY 密钥对
#[derive(Debug, Clone)]
pub struct RealityKeyPair {
    /// 私钥（Base64 编码）
    pub private_key: String,
    /// 公钥（Base64 编码）
    pub public_key: String,
}

/// 生成 REALITY 密钥对
/// 注意：这是一个简化实现，生产环境建议使用 `sing-box generate reality-keypair`
pub fn generate_reality_keypair() -> RealityKeyPair {
    use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
    use rand::RngCore;

    // 生成 32 字节的随机私钥
    let mut private_key_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut private_key_bytes);

    // X25519 密钥生成
    // 按照 X25519 规范处理私钥
    private_key_bytes[0] &= 248;
    private_key_bytes[31] &= 127;
    private_key_bytes[31] |= 64;

    // 使用 x25519_dalek 计算公钥
    let private_key = x25519_dalek::StaticSecret::from(private_key_bytes);
    let public_key = x25519_dalek::PublicKey::from(&private_key);

    RealityKeyPair {
        private_key: URL_SAFE_NO_PAD.encode(private_key_bytes),
        public_key: URL_SAFE_NO_PAD.encode(public_key.as_bytes()),
    }
}

/// 生成 REALITY 短 ID（8位十六进制）
pub fn generate_short_id() -> String {
    generate_hex_string(4) // 4 bytes = 8 hex chars
}

/// 多协议自动配置结果
#[derive(Debug)]
pub struct MultiProtocolResult {
    /// 服务器公网 IP
    pub public_ip: IpAddr,
    /// 使用的域名
    pub domain: String,
    /// AnyTLS 配置（如果启用）
    pub anytls: Option<AnyTlsAutoResult>,
    /// Hysteria2 配置（如果启用）
    pub hysteria2: Option<Hysteria2AutoResult>,
    /// TUIC 配置（如果启用）
    pub tuic: Option<TuicAutoResult>,
    /// VLESS-Reality 配置（如果启用）
    pub vless_reality: Option<VlessRealityAutoResult>,
}

//============================================================================
// 错误类型
//============================================================================

/// 自动配置错误
#[derive(Debug, Clone)]
pub enum AutoDefaultError {
    /// 获取公网 IP 失败
    PublicIpError(String),
    /// 无可用端口
    NoAvailablePort,
    /// 配置生成失败
    ConfigError(String),
}

impl std::fmt::Display for AutoDefaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoDefaultError::PublicIpError(msg) => write!(f, "获取公网 IP 失败: {}", msg),
            AutoDefaultError::NoAvailablePort => write!(f, "无可用端口"),
            AutoDefaultError::ConfigError(msg) => write!(f, "配置生成失败: {}", msg),
        }
    }
}

impl std::error::Error for AutoDefaultError {}

impl From<PublicIpError> for AutoDefaultError {
    fn from(err: PublicIpError) -> Self {
        AutoDefaultError::PublicIpError(err.to_string())
    }
}

//============================================================================
// 自动配置构建器
//============================================================================

/// 全自动配置构建器
///
/// # 示例
///
/// ```ignore
/// // 最简单的用法 - 完全自动化
/// let result = AutoDefault::anytls().build()?;
///
/// // 自定义用户
/// let result = AutoDefault::hysteria2()
///     .add_user("user1")
///     .add_user_with_password("user2", "my_password")
///     .build()?;
///
/// // 指定端口
/// let result = AutoDefault::tuic()
///     .port(2053)
///     .build()?;
/// ```
#[derive(Debug)]
pub struct AutoDefault {
    /// 协议类型
    protocol: Protocol,
    /// 指定的公网 IP（如果不指定则自动获取）
    public_ip: Option<IpAddr>,
    /// 指定域名（不指定则使用基于公网 IP 的 sslip.io）
    domain: Option<String>,
    /// 指定的端口（如果不指定则使用默认端口）
    port: Option<u16>,
    /// 用户列表
    users: Vec<GeneratedUser>,
    /// 入站标签
    tag: Option<String>,
    /// Hysteria2 特有：上行带宽
    up_mbps: Option<u32>,
    /// Hysteria2 特有：下行带宽
    down_mbps: Option<u32>,
    /// Hysteria2 特有：启用混淆
    enable_obfs: bool,
    /// Hysteria2 特有：伪装 URL
    masquerade_url: Option<String>,
    /// TUIC 特有：拥塞控制算法
    congestion_control: Option<CongestionControl>,
    /// VLESS Reality 特有：握手服务器
    reality_handshake_server: Option<String>,
    /// VLESS Reality 特有：握手服务器端口
    reality_handshake_port: Option<u16>,
    /// VLESS Reality 特有：服务器名称（SNI）
    reality_server_name: Option<String>,
}

impl AutoDefault {
    /// 创建AnyTLS 自动配置
    pub fn anytls() -> Self {
        Self::new(Protocol::AnyTls)
    }

    /// 创建 Hysteria2 自动配置
    pub fn hysteria2() -> Self {
        Self::new(Protocol::Hysteria2)
    }

    /// 创建 TUIC 自动配置
    pub fn tuic() -> Self {
        Self::new(Protocol::Tuic)
    }

    /// 创建 VLESS-Vision-uTLS-REALITY 自动配置
    pub fn vless_reality() -> Self {
        Self::new(Protocol::VlessReality)
    }

    /// 创建指定协议的自动配置
    fn new(protocol: Protocol) -> Self {
        Self {
            protocol,
            public_ip: None,
            domain: None,
            port: None,
            users: Vec::new(),
            tag: None,
            up_mbps: None,
            down_mbps: None,
            enable_obfs: false,
            masquerade_url: None,
            congestion_control: None,
            reality_handshake_server: None,
            reality_handshake_port: None,
            reality_server_name: None,
        }
    }

    /// 设置公网 IP（不设置则自动获取）
    pub fn public_ip(mut self, ip: IpAddr) -> Self {
        self.public_ip = Some(ip);
        self
    }

    /// 设置域名（不指定则使用基于公网 IP 的 sslip.io）
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// 从字符串设置公网 IP
    pub fn public_ip_str(mut self, ip: &str) -> Result<Self, std::net::AddrParseError> {
        self.public_ip = Some(ip.parse()?);
        Ok(self)
    }

    /// 设置端口
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// 使用备用端口（按索引）
    pub fn fallback_port(mut self, index: usize) -> Self {
        self.port = Some(fallback_port(index));
        self
    }

    /// 设置入站标签
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// 添加用户（自动生成密码/UUID）
    pub fn add_user(mut self, name: impl Into<String>) -> Self {
        let user = if self.protocol == Protocol::Tuic || self.protocol == Protocol::VlessReality {
            GeneratedUser::with_uuid(name)
        } else {
            GeneratedUser::new(name)
        };
        self.users.push(user);
        self
    }

    /// 添加用户（指定密码）
    pub fn add_user_with_password(
        mut self,
        name: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        let mut user = GeneratedUser::with_password(name, password);
        if self.protocol == Protocol::Tuic || self.protocol == Protocol::VlessReality {
            user.uuid = Some(generate_uuid());
        }
        self.users.push(user);
        self
    }

    /// 添加 TUIC 用户（指定 UUID）
    pub fn add_tuic_user(
        mut self,
        name: impl Into<String>,
        uuid: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.users
            .push(GeneratedUser::with_credentials(name, password, uuid));
        self
    }

    //========== Hysteria2 特有方法 ==========

    /// 设置带宽限制（Hysteria2）
    pub fn bandwidth(mut self, up_mbps: u32, down_mbps: u32) -> Self {
        self.up_mbps = Some(up_mbps);
        self.down_mbps = Some(down_mbps);
        self
    }

    /// 启用混淆（Hysteria2）
    pub fn with_obfs(mut self) -> Self {
        self.enable_obfs = true;
        self
    }

    /// 设置伪装 URL（Hysteria2）
    pub fn masquerade(mut self, url: impl Into<String>) -> Self {
        self.masquerade_url = Some(url.into());
        self
    }

    // ========== TUIC 特有方法 ==========

    /// 使用 BBR 拥塞控制（TUIC）
    pub fn bbr(mut self) -> Self {
        self.congestion_control = Some(CongestionControl::Bbr);
        self
    }

    /// 使用 Cubic 拥塞控制（TUIC）
    pub fn cubic(mut self) -> Self {
        self.congestion_control = Some(CongestionControl::Cubic);
        self
    }

    /// 使用 New Reno 拥塞控制（TUIC）
    pub fn new_reno(mut self) -> Self {
        self.congestion_control = Some(CongestionControl::NewReno);
        self
    }

    // ========== VLESS Reality 特有方法 ==========

    /// 设置 REALITY 握手服务器（VLESS Reality）
    /// 默认使用 www.microsoft.com:443
    pub fn handshake_server(mut self, server: impl Into<String>, port: u16) -> Self {
        self.reality_handshake_server = Some(server.into());
        self.reality_handshake_port = Some(port);
        self
    }

    /// 设置 REALITY 服务器名称/SNI（VLESS Reality）
    /// 默认使用握手服务器地址
    pub fn server_name(mut self, name: impl Into<String>) -> Self {
        self.reality_server_name = Some(name.into());
        self
    }

    // ========== 构建方法 ==========

    /// 获取或自动检测公网 IP
    fn get_public_ip(&self) -> Result<IpAddr, AutoDefaultError> {
        if let Some(ip) = self.public_ip {
            Ok(ip)
        } else {
            Ok(get_public_ip()?)
        }
    }

    /// 生成 TLS 配置
    fn generate_tls_config(&self, domain: &str) -> InboundTlsConfig {
        let acme = AcmeConfig {
            domain: Some(vec![domain.to_string()]),
            email: None,
            ..Default::default()
        };

        InboundTlsConfig {
            enabled: Some(true),
            server_name: Some(domain.to_string()),
            acme: Some(acme),
            ..Default::default()
        }
    }

    /// 生成用户列表（如果为空则生成默认用户）
    fn generate_users(&self) -> Vec<GeneratedUser> {
        if self.users.is_empty() {
            let user = if self.protocol == Protocol::Tuic || self.protocol == Protocol::VlessReality
            {
                GeneratedUser::with_uuid("default")
            } else {
                GeneratedUser::new("default")
            };
            vec![user]
        } else {
            self.users.clone()
        }
    }

    /// 构建 AnyTLS 配置
    pub fn build_anytls(self) -> Result<AnyTlsAutoResult, AutoDefaultError> {
        let public_ip = self.get_public_ip()?;
        let domain = self
            .domain
            .clone()
            .unwrap_or_else(|| generate_sslip_domain(&public_ip));
        let port = self.port.unwrap_or_else(default_port);
        let tag = self
            .tag
            .clone()
            .unwrap_or_else(|| Protocol::AnyTls.default_tag().to_string());
        let users = self.generate_users();
        let tls = self.generate_tls_config(&domain);

        let mut inbound = AnyTlsInbound::new(&tag)
            .with_listen("::")
            .with_listen_port(port)
            .with_tls(tls);

        for user in &users {
            inbound = inbound.add_user(&user.name, &user.password);
        }

        Ok(AnyTlsAutoResult {
            info: AutoDefaultResult {
                public_ip,
                domain,
                port,
                users,
            },
            inbound,
        })
    }

    /// 构建 Hysteria2 配置
    pub fn build_hysteria2(self) -> Result<Hysteria2AutoResult, AutoDefaultError> {
        let public_ip = self.get_public_ip()?;
        let domain = self
            .domain
            .clone()
            .unwrap_or_else(|| generate_sslip_domain(&public_ip));
        let port = self.port.unwrap_or_else(default_port);
        let tag = self
            .tag
            .clone()
            .unwrap_or_else(|| Protocol::Hysteria2.default_tag().to_string());
        let users = self.generate_users();
        let tls = self.generate_tls_config(&domain);

        let mut inbound = Hysteria2Inbound::new(&tag)
            .with_listen("::")
            .with_listen_port(port)
            .with_tls(tls);

        for user in &users {
            inbound = inbound.add_user(&user.name, &user.password);
        }

        // 带宽限制
        if let (Some(up), Some(down)) = (self.up_mbps, self.down_mbps) {
            inbound = inbound.with_bandwidth(up, down);
        }

        // 混淆
        let obfs_password = if self.enable_obfs {
            let pwd = generate_password();
            inbound = inbound.with_obfs(&pwd);
            Some(pwd)
        } else {
            None
        };

        // 伪装
        if let Some(ref url) = self.masquerade_url {
            inbound = inbound.with_masquerade_url(url);
        }

        Ok(Hysteria2AutoResult {
            info: AutoDefaultResult {
                public_ip,
                domain,
                port,
                users,
            },
            inbound,
            obfs_password,
        })
    }

    /// 构建 TUIC 配置
    pub fn build_tuic(self) -> Result<TuicAutoResult, AutoDefaultError> {
        let public_ip = self.get_public_ip()?;
        let domain = self
            .domain
            .clone()
            .unwrap_or_else(|| generate_sslip_domain(&public_ip));
        let port = self.port.unwrap_or_else(default_port);
        let tag = self
            .tag
            .clone()
            .unwrap_or_else(|| Protocol::Tuic.default_tag().to_string());
        let users = self.generate_users();
        let tls = self.generate_tls_config(&domain);

        let cc = self.congestion_control.unwrap_or(CongestionControl::Cubic);

        let mut inbound = TuicInbound::new(&tag)
            .with_listen("::")
            .with_listen_port(port)
            .with_tls(tls)
            .with_congestion_control(cc);

        for user in &users {
            let tuic_user = if let Some(ref uuid) = user.uuid {
                TuicUser::with_credentials(&user.name, uuid, &user.password)
            } else {
                TuicUser::with_credentials(&user.name, generate_uuid(), &user.password)
            };
            inbound = inbound.add_user(tuic_user);
        }

        Ok(TuicAutoResult {
            info: AutoDefaultResult {
                public_ip,
                domain,
                port,
                users,
            },
            inbound,
        })
    }

    /// 构建 VLESS-Vision-uTLS-REALITY 配置
    pub fn build_vless_reality(self) -> Result<VlessRealityAutoResult, AutoDefaultError> {
        let public_ip = self.get_public_ip()?;
        let port = self.port.unwrap_or_else(default_port);
        let tag = self
            .tag
            .clone()
            .unwrap_or_else(|| Protocol::VlessReality.default_tag().to_string());
        let users = self.generate_users();

        // REALITY 配置
        let handshake_server = self
            .reality_handshake_server
            .clone()
            .unwrap_or_else(|| "www.microsoft.com".to_string());
        let handshake_port = self.reality_handshake_port.unwrap_or(443);
        let server_name = self
            .reality_server_name
            .clone()
            .unwrap_or_else(|| handshake_server.clone());

        // 生成 REALITY 密钥对
        let keypair = generate_reality_keypair();
        let short_id = generate_short_id();

        // 构建 REALITY TLS 配置
        let reality_config = RealityInboundConfig {
            enabled: Some(true),
            handshake: Some(RealityHandshake {
                server: handshake_server.clone(),
                server_port: Some(handshake_port),
                bind_interface: None,
                inet4_bind_address: None,
                inet6_bind_address: None,
                routing_mark: None,
                reuse_addr: None,
                netns: None,
                connect_timeout: None,
                tcp_fast_open: None,
                tcp_multi_path: None,
                udp_fragment: None,
                domain_strategy: None,
                fallback_delay: None,
            }),
            private_key: Some(keypair.private_key.clone()),
            short_id: Some(vec![short_id.clone()]),
            max_time_difference: None,
        };

        let tls_config = InboundTlsConfig {
            enabled: Some(true),
            server_name: Some(server_name),
            reality: Some(reality_config),
            ..Default::default()
        };

        // 构建入站配置
        let mut inbound = VlessInbound::new(&tag)
            .with_listen("::")
            .with_listen_port(port)
            .with_tls(tls_config);

        // 添加用户（带XTLS Vision flow）
        for user in &users {
            let uuid = user.uuid.clone().unwrap_or_else(generate_uuid);
            let vless_user = VlessUser::new(&user.name, &uuid).with_xtls_vision();
            inbound = inbound.add_user(vless_user);
        }

        // 使用公网 IP 作为域名（REALITY 不需要真实域名）
        let domain = public_ip.to_string();

        Ok(VlessRealityAutoResult {
            info: AutoDefaultResult {
                public_ip,
                domain,
                port,
                users,
            },
            inbound,
            private_key: keypair.private_key,
            public_key: keypair.public_key,
            short_id,
            handshake_server,
            handshake_port,
        })
    }

    /// 根据协议类型自动构建
    pub fn build(self) -> Result<AutoBuildResult, AutoDefaultError> {
        match self.protocol {
            Protocol::AnyTls => Ok(AutoBuildResult::AnyTls(self.build_anytls()?)),
            Protocol::Hysteria2 => Ok(AutoBuildResult::Hysteria2(self.build_hysteria2()?)),
            Protocol::Tuic => Ok(AutoBuildResult::Tuic(self.build_tuic()?)),
            Protocol::VlessReality => {
                Ok(AutoBuildResult::VlessReality(self.build_vless_reality()?))
            }
        }
    }
}

/// 自动构建结果枚举
#[derive(Debug)]
pub enum AutoBuildResult {
    /// AnyTLS 结果
    AnyTls(AnyTlsAutoResult),
    /// Hysteria2 结果
    Hysteria2(Hysteria2AutoResult),
    /// TUIC 结果
    Tuic(TuicAutoResult),
    /// VLESS-Reality 结果
    VlessReality(VlessRealityAutoResult),
}

//============================================================================
// 多协议构建器
//============================================================================

/// 多协议自动配置构建器
///
/// # 示例
///
/// ```ignore
/// // 同时部署三种协议
/// let result = MultiProtocolBuilder::new()
///     .enable_anytls(443)
///     .enable_hysteria2(2053)
///     .enable_tuic(2083)
///     .add_user("user1")
///     .build()?;
/// ```
#[derive(Debug)]
pub struct MultiProtocolBuilder {
    /// 公网 IP
    public_ip: Option<IpAddr>,
    /// 指定域名（不指定则使用基于公网 IP 的 sslip.io）
    domain: Option<String>,
    /// 用户列表
    users: Vec<GeneratedUser>,
    /// AnyTLS 端口（None表示不启用）
    anytls_port: Option<u16>,
    /// Hysteria2 端口
    hysteria2_port: Option<u16>,
    /// TUIC 端口
    tuic_port: Option<u16>,
    /// VLESS Reality 端口
    vless_reality_port: Option<u16>,
    /// Hysteria2 带宽
    hy2_bandwidth: Option<(u32, u32)>,
    /// Hysteria2 混淆
    hy2_obfs: bool,
    /// TUIC 拥塞控制
    tuic_cc: Option<CongestionControl>,
    /// VLESS Reality 握手服务器
    vless_handshake: Option<(String, u16)>,
}

impl MultiProtocolBuilder {
    /// 创建新的多协议构建器
    pub fn new() -> Self {
        Self {
            public_ip: None,
            domain: None,
            users: Vec::new(),
            anytls_port: None,
            hysteria2_port: None,
            tuic_port: None,
            vless_reality_port: None,
            hy2_bandwidth: None,
            hy2_obfs: false,
            tuic_cc: None,
            vless_handshake: None,
        }
    }

    /// 设置公网 IP
    pub fn public_ip(mut self, ip: IpAddr) -> Self {
        self.public_ip = Some(ip);
        self
    }

    /// 设置域名（不指定则使用基于公网 IP 的 sslip.io）
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// 启用 AnyTLS
    pub fn enable_anytls(mut self, port: u16) -> Self {
        self.anytls_port = Some(port);
        self
    }

    /// 启用 Hysteria2
    pub fn enable_hysteria2(mut self, port: u16) -> Self {
        self.hysteria2_port = Some(port);
        self
    }

    /// 启用 TUIC
    pub fn enable_tuic(mut self, port: u16) -> Self {
        self.tuic_port = Some(port);
        self
    }

    /// 启用 VLESS-Reality
    pub fn enable_vless_reality(mut self, port: u16) -> Self {
        self.vless_reality_port = Some(port);
        self
    }

    /// 设置 VLESS Reality 握手服务器
    pub fn vless_handshake(mut self, server: impl Into<String>, port: u16) -> Self {
        self.vless_handshake = Some((server.into(), port));
        self
    }

    /// 启用所有协议（使用默认端口）
    pub fn enable_all(mut self) -> Self {
        self.anytls_port = Some(DEFAULT_PORTS[0]); // 443
        self.hysteria2_port = Some(DEFAULT_PORTS[1]); // 2053
        self.tuic_port = Some(DEFAULT_PORTS[2]); // 2083
        self.vless_reality_port = Some(DEFAULT_PORTS[3]); // 2096
        self
    }

    /// 添加用户
    pub fn add_user(mut self, name: impl Into<String>) -> Self {
        self.users.push(GeneratedUser::with_uuid(name));
        self
    }

    /// 添加用户（指定密码）
    pub fn add_user_with_password(
        mut self,
        name: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        let mut user = GeneratedUser::with_password(name, password);
        user.uuid = Some(generate_uuid());
        self.users.push(user);
        self
    }

    /// 设置 Hysteria2 带宽
    pub fn hy2_bandwidth(mut self, up_mbps: u32, down_mbps: u32) -> Self {
        self.hy2_bandwidth = Some((up_mbps, down_mbps));
        self
    }

    /// 启用 Hysteria2 混淆
    pub fn hy2_obfs(mut self) -> Self {
        self.hy2_obfs = true;
        self
    }

    /// 设置 TUIC 拥塞控制
    pub fn tuic_congestion(mut self, cc: CongestionControl) -> Self {
        self.tuic_cc = Some(cc);
        self
    }

    /// 构建多协议配置
    pub fn build(self) -> Result<MultiProtocolResult, AutoDefaultError> {
        let public_ip = if let Some(ip) = self.public_ip {
            ip
        } else {
            get_public_ip()?
        };

        let domain = self
            .domain
            .clone()
            .unwrap_or_else(|| generate_sslip_domain(&public_ip));
        let users = if self.users.is_empty() {
            vec![GeneratedUser::with_uuid("default")]
        } else {
            self.users
        };

        //构建 AnyTLS
        let anytls = if let Some(port) = self.anytls_port {
            let mut builder = AutoDefault::anytls()
                .public_ip(public_ip)
                .domain(domain.clone())
                .port(port);
            for user in &users {
                builder = builder.add_user_with_password(&user.name, &user.password);
            }
            Some(builder.build_anytls()?)
        } else {
            None
        };

        // 构建 Hysteria2
        let hysteria2 = if let Some(port) = self.hysteria2_port {
            let mut builder = AutoDefault::hysteria2()
                .public_ip(public_ip)
                .domain(domain.clone())
                .port(port);
            for user in &users {
                builder = builder.add_user_with_password(&user.name, &user.password);
            }
            if let Some((up, down)) = self.hy2_bandwidth {
                builder = builder.bandwidth(up, down);
            }
            if self.hy2_obfs {
                builder = builder.with_obfs();
            }
            Some(builder.build_hysteria2()?)
        } else {
            None
        };

        // 构建 TUIC
        let tuic = if let Some(port) = self.tuic_port {
            let mut builder = AutoDefault::tuic()
                .public_ip(public_ip)
                .domain(domain.clone())
                .port(port);
            for user in &users {
                if let Some(ref uuid) = user.uuid {
                    builder = builder.add_tuic_user(&user.name, uuid, &user.password);
                } else {
                    builder = builder.add_user_with_password(&user.name, &user.password);
                }
            }
            if let Some(cc) = self.tuic_cc {
                builder = match cc {
                    CongestionControl::Bbr => builder.bbr(),
                    CongestionControl::Cubic => builder.cubic(),
                    CongestionControl::NewReno => builder.new_reno(),
                };
            }
            Some(builder.build_tuic()?)
        } else {
            None
        };

        // 构建 VLESS Reality
        let vless_reality = if let Some(port) = self.vless_reality_port {
            let mut builder = AutoDefault::vless_reality().public_ip(public_ip).port(port);
            for user in &users {
                if let Some(ref uuid) = user.uuid {
                    builder = builder.add_tuic_user(&user.name, uuid, &user.password);
                } else {
                    builder = builder.add_user(&user.name);
                }
            }
            if let Some((server, hs_port)) = &self.vless_handshake {
                builder = builder.handshake_server(server, *hs_port);
            }
            Some(builder.build_vless_reality()?)
        } else {
            None
        };

        Ok(MultiProtocolResult {
            public_ip,
            domain,
            anytls,
            hysteria2,
            tuic,
            vless_reality,
        })
    }
}

impl Default for MultiProtocolBuilder {
    fn default() -> Self {
        Self::new()
    }
}

//============================================================================
// 便捷函数
//============================================================================

/// 快速创建 AnyTLS 配置（完全自动化）
pub fn quick_anytls() -> Result<AnyTlsAutoResult, AutoDefaultError> {
    AutoDefault::anytls().build_anytls()
}

/// 快速创建 Hysteria2 配置（完全自动化）
pub fn quick_hysteria2() -> Result<Hysteria2AutoResult, AutoDefaultError> {
    AutoDefault::hysteria2().build_hysteria2()
}

/// 快速创建 TUIC 配置（完全自动化）
pub fn quick_tuic() -> Result<TuicAutoResult, AutoDefaultError> {
    AutoDefault::tuic().build_tuic()
}

/// 快速创建所有协议配置（完全自动化）
pub fn quick_all() -> Result<MultiProtocolResult, AutoDefaultError> {
    MultiProtocolBuilder::new().enable_all().build()
}

/// 快速创建 VLESS-Reality 配置（完全自动化）
pub fn quick_vless_reality() -> Result<VlessRealityAutoResult, AutoDefaultError> {
    AutoDefault::vless_reality().build_vless_reality()
}

//============================================================================
// 单元测试
//============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ip() -> IpAddr {
        "1.2.3.4".parse().unwrap()
    }

    #[test]
    fn test_default_ports() {
        assert_eq!(default_port(), 443);
        assert_eq!(fallback_port(0), 443);
        assert_eq!(fallback_port(1), 2053);
        assert_eq!(fallback_port(2), 2083);
        assert_eq!(fallback_port(100), 443); // 超出范围返回默认
    }

    #[test]
    fn test_generated_user() {
        let user = GeneratedUser::new("test");
        assert_eq!(user.name, "test");
        assert!(!user.password.is_empty());
        assert!(user.uuid.is_none());

        let user_with_uuid = GeneratedUser::with_uuid("test2");
        assert!(user_with_uuid.uuid.is_some());
    }

    #[test]
    fn test_auto_anytls() {
        let result = AutoDefault::anytls()
            .public_ip(test_ip())
            .port(443)
            .add_user("user1")
            .build_anytls()
            .unwrap();

        assert_eq!(result.info.port, 443);
        assert_eq!(result.info.domain, "1-2-3-4.sslip.io");
        assert_eq!(result.info.users.len(), 1);
        assert_eq!(result.inbound.inbound_type, "anytls");
    }

    #[test]
    fn test_auto_hysteria2() {
        let result = AutoDefault::hysteria2()
            .public_ip(test_ip())
            .port(2053)
            .add_user("user1")
            .bandwidth(100, 100)
            .with_obfs()
            .build_hysteria2()
            .unwrap();
        assert_eq!(result.info.port, 2053);
        assert_eq!(result.inbound.inbound_type, "hysteria2");
        assert!(result.obfs_password.is_some());
        assert_eq!(result.inbound.up_mbps, Some(100));
        assert_eq!(result.inbound.down_mbps, Some(100));
    }

    #[test]
    fn test_auto_tuic() {
        let result = AutoDefault::tuic()
            .public_ip(test_ip())
            .port(2083)
            .add_user("user1")
            .bbr()
            .build_tuic()
            .unwrap();

        assert_eq!(result.info.port, 2083);
        assert_eq!(result.inbound.inbound_type, "tuic");
        assert_eq!(
            result.inbound.congestion_control,
            Some(CongestionControl::Bbr)
        );
        // TUIC 用户应该有 UUID
        assert!(result.info.users[0].uuid.is_some());
    }

    #[test]
    fn test_auto_build_generic() {
        let result = AutoDefault::anytls().public_ip(test_ip()).build().unwrap();

        assert!(matches!(result, AutoBuildResult::AnyTls(_)));
    }

    #[test]
    fn test_multi_protocol_builder() {
        let result = MultiProtocolBuilder::new()
            .public_ip(test_ip())
            .enable_anytls(443)
            .enable_hysteria2(2053)
            .enable_tuic(2083)
            .add_user("user1")
            .build()
            .unwrap();

        assert!(result.anytls.is_some());
        assert!(result.hysteria2.is_some());
        assert!(result.tuic.is_some());

        assert_eq!(result.anytls.as_ref().unwrap().info.port, 443);
        assert_eq!(result.hysteria2.as_ref().unwrap().info.port, 2053);
        assert_eq!(result.tuic.as_ref().unwrap().info.port, 2083);
    }

    #[test]
    fn test_multi_protocol_enable_all() {
        let result = MultiProtocolBuilder::new()
            .public_ip(test_ip())
            .enable_all()
            .build()
            .unwrap();

        assert!(result.anytls.is_some());
        assert!(result.hysteria2.is_some());
        assert!(result.tuic.is_some());
    }

    #[test]
    fn test_default_user_generation() {
        let result = AutoDefault::anytls()
            .public_ip(test_ip())
            .build_anytls()
            .unwrap();

        // 没有添加用户时应该自动生成默认用户
        assert_eq!(result.info.users.len(), 1);
        assert_eq!(result.info.users[0].name, "default");
    }

    #[test]
    fn test_serialize_inbound() {
        let result = AutoDefault::anytls()
            .public_ip(test_ip())
            .port(443)
            .add_user("sekai")
            .build_anytls()
            .unwrap();

        let json = serde_json::to_string_pretty(&result.inbound).unwrap();
        assert!(json.contains("\"type\": \"anytls\""));
        assert!(json.contains("\"listen_port\": 443"));
        assert!(json.contains("1-2-3-4.sslip.io"));
    }

    #[test]
    fn test_protocol_default_tags() {
        assert_eq!(Protocol::AnyTls.default_tag(), "anytls-in");
        assert_eq!(Protocol::Hysteria2.default_tag(), "hy2-in");
        assert_eq!(Protocol::Tuic.default_tag(), "tuic-in");
        assert_eq!(Protocol::VlessReality.default_tag(), "vless-reality-in");
    }

    #[test]
    fn test_auto_vless_reality() {
        let result = AutoDefault::vless_reality()
            .public_ip(test_ip())
            .port(443)
            .add_user("user1")
            .build_vless_reality()
            .unwrap();

        assert_eq!(result.info.port, 443);
        assert_eq!(result.info.users.len(), 1);
        assert_eq!(result.inbound.inbound_type, "vless");
        //验证 REALITY 密钥对已生成
        assert!(!result.private_key.is_empty());
        assert!(!result.public_key.is_empty());
        assert!(!result.short_id.is_empty());
        // 验证握手服务器默认值
        assert_eq!(result.handshake_server, "www.microsoft.com");
        assert_eq!(result.handshake_port, 443);
    }

    #[test]
    fn test_vless_reality_custom_handshake() {
        let result = AutoDefault::vless_reality()
            .public_ip(test_ip())
            .port(443)
            .handshake_server("www.google.com", 443)
            .server_name("www.google.com")
            .add_user("user1")
            .build_vless_reality()
            .unwrap();

        assert_eq!(result.handshake_server, "www.google.com");
        assert_eq!(result.handshake_port, 443);
    }

    #[test]
    fn test_reality_keypair_generation() {
        let keypair1 = generate_reality_keypair();
        let keypair2 = generate_reality_keypair();

        // 每次生成的密钥对应该不同
        assert_ne!(keypair1.private_key, keypair2.private_key);
        assert_ne!(keypair1.public_key, keypair2.public_key);

        // 密钥应该是有效的 Base64 编码
        assert!(!keypair1.private_key.is_empty());
        assert!(!keypair1.public_key.is_empty());
    }

    #[test]
    fn test_short_id_generation() {
        let short_id = generate_short_id();
        // 短ID 应该是 8 位十六进制字符
        assert_eq!(short_id.len(), 8);
        assert!(short_id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_multi_protocol_with_vless_reality() {
        let result = MultiProtocolBuilder::new()
            .public_ip(test_ip())
            .enable_vless_reality(2096)
            .add_user("user1")
            .build()
            .unwrap();

        assert!(result.vless_reality.is_some());
        let vless = result.vless_reality.unwrap();
        assert_eq!(vless.info.port, 2096);
    }

    #[test]
    fn test_custom_tag() {
        let result = AutoDefault::anytls()
            .public_ip(test_ip())
            .tag("my-custom-tag")
            .build_anytls()
            .unwrap();

        assert_eq!(result.inbound.tag, "my-custom-tag");
    }

    #[test]
    fn test_fallback_port_usage() {
        let result = AutoDefault::anytls()
            .public_ip(test_ip())
            .fallback_port(1) // 2053
            .build_anytls()
            .unwrap();

        assert_eq!(result.info.port, 2053);
    }
}
