use serde::{Deserialize, Serialize};

use crate::singboxconfig::types::{DomainStrategy, Duration, RoutingMark, StringOrArray};

//============================================================================
// 入站 TLS 配置（服务端）
// ============================================================================

/// 入站 TLS 配置（服务端）
/// 文档: https://sing-box.sagernet.org/configuration/shared/tls/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InboundTlsConfig {
    /// 启用 TLS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 服务器名称，用于验证返回证书的主机名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,

    /// 支持的应用层协议列表，按优先级排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,

    /// 可接受的最低 TLS 版本
    /// 作为服务端时默认为 TLS 1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<TlsVersion>,

    /// 可接受的最高 TLS 版本
    /// 默认为 TLS 1.3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_version: Option<TlsVersion>,

    /// 启用的TLS 1.0-1.2 密码套件列表
    /// TLS 1.3 密码套件不可配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<Vec<CipherSuite>>,

    /// 支持的密钥交换机制集合
    ///自sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve_preferences: Option<Vec<CurvePreference>>,

    /// 服务器证书链行数组，PEM 格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<StringOrArray>,

    /// 服务器证书链路径，PEM 格式
    /// 文件修改时会自动重新加载
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_path: Option<String>,

    /// 客户端认证类型
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<ClientAuthentication>,

    /// 客户端证书链行数组，PEM 格式（用于验证客户端）
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<StringOrArray>,

    /// 客户端证书链路径列表，PEM 格式
    /// 文件修改时会自动重新加载
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_path: Option<Vec<String>>,

    /// 客户端证书公钥的 SHA-256 哈希列表，base64 格式
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_public_key_sha256: Option<Vec<String>>,

    /// 服务器私钥行数组，PEM 格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<StringOrArray>,

    /// 服务器私钥路径，PEM 格式
    /// 文件修改时会自动重新加载
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,

    /// 启用内核 TLS 发送支持
    ///仅支持 Linux 5.1+，仅支持 TLS 1.3
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_tx: Option<bool>,

    /// 启用内核 TLS 接收支持
    /// 仅支持 Linux 5.1+，仅支持 TLS 1.3
    /// 不建议启用，会降低性能
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_rx: Option<bool>,

    /// ACME 自动证书配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acme: Option<AcmeConfig>,

    /// ECH（加密客户端 Hello）配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ech: Option<EchInboundConfig>,

    /// Reality配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality: Option<RealityInboundConfig>,
}

// ============================================================================
// 出站 TLS 配置（客户端）
// ============================================================================

/// 出站 TLS 配置（客户端）
/// 文档: https://sing-box.sagernet.org/configuration/shared/tls/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutboundTlsConfig {
    /// 启用 TLS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 不在 ClientHello 中发送服务器名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_sni: Option<bool>,

    /// 服务器名称，用于验证返回证书的主机名
    /// 除非是 IP 地址，否则会包含在客户端握手中以支持虚拟主机
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,

    /// 接受任何服务器证书（不安全）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,

    /// 支持的应用层协议列表，按优先级排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,

    /// 可接受的最低 TLS 版本
    /// 作为客户端时默认为 TLS 1.2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<TlsVersion>,

    /// 可接受的最高 TLS 版本
    /// 默认为 TLS 1.3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_version: Option<TlsVersion>,

    /// 启用的 TLS 1.0-1.2 密码套件列表
    /// TLS 1.3 密码套件不可配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<Vec<CipherSuite>>,

    /// 支持的密钥交换机制集合
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve_preferences: Option<Vec<CurvePreference>>,

    /// 服务器证书，PEM 格式（用于证书固定）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<StringOrArray>,

    /// 服务器证书路径，PEM 格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_path: Option<String>,

    /// 服务器证书公钥的 SHA-256 哈希列表，base64 格式
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_public_key_sha256: Option<Vec<String>>,

    /// 客户端证书链行数组，PEM 格式
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<StringOrArray>,

    /// 客户端证书链路径，PEM 格式
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_path: Option<String>,

    /// 客户端私钥行数组，PEM 格式
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_key: Option<StringOrArray>,

    /// 客户端私钥路径，PEM 格式
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_key_path: Option<String>,

    /// 分片TLS 握手以绕过防火墙
    /// 自 sing-box 1.12.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment: Option<bool>,

    /// 无法自动确定等待时间时使用的回退值
    /// 默认值: 500ms
    /// 自 sing-box 1.12.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_fallback_delay: Option<Duration>,

    /// 将 TLS 握手分片为多个 TLS 记录以绕过防火墙
    /// 自 sing-box 1.12.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_fragment: Option<bool>,

    /// ECH（加密客户端 Hello）配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ech: Option<EchOutboundConfig>,

    /// uTLS 配置，用于 ClientHello 指纹伪装
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utls: Option<UtlsConfig>,

    /// Reality 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality: Option<RealityOutboundConfig>,
}

// ============================================================================
// ECH 配置
// ============================================================================

/// ECH 入站配置（服务端）
/// ECH（加密客户端 Hello）是一种 TLS 扩展，允许客户端加密 ClientHello 消息的第一部分
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EchInboundConfig {
    /// 启用 ECH
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// ECH 密钥行数组，PEM 格式
    /// 可通过 `sing-box generateech-keypair` 生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<StringOrArray>,

    /// ECH 密钥路径，PEM 格式
    /// 文件修改时会自动重新加载
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,

    /// 启用后量子对等证书签名方案支持
    /// 自 sing-box 1.12.0 起已弃用（不再有效）
    #[deprecated(
        since = "1.12.0",
        note = "ECH 已迁移到使用 stdlib，不再支持 PQ 签名方案"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pq_signature_schemes_enabled: Option<bool>,

    ///禁用 TLS 记录的自适应大小调整
    /// 自 sing-box 1.12.0 起已弃用（不再有效）
    #[deprecated(since = "1.12.0", note = "此选项与 ECH 无关，已被错误添加")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_record_sizing_disabled: Option<bool>,
}

/// ECH 出站配置（客户端）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EchOutboundConfig {
    /// 启用 ECH
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// ECH 配置行数组，PEM 格式
    /// 如果为空，将尝试从 DNS 加载
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<StringOrArray>,

    /// ECH 配置路径，PEM 格式
    /// 如果为空，将尝试从 DNS 加载
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_path: Option<String>,

    /// 启用后量子对等证书签名方案支持
    /// 自 sing-box 1.12.0 起已弃用（不再有效）
    #[deprecated(
        since = "1.12.0",
        note = "ECH 已迁移到使用 stdlib，不再支持 PQ 签名方案"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pq_signature_schemes_enabled: Option<bool>,

    /// 禁用 TLS 记录的自适应大小调整
    /// 自 sing-box 1.12.0 起已弃用（不再有效）
    #[deprecated(since = "1.12.0", note = "此选项与 ECH 无关，已被错误添加")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_record_sizing_disabled: Option<bool>,
}

// ============================================================================
// Reality 配置
// ============================================================================

/// Reality 入站配置（服务端）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RealityInboundConfig {
    /// 启用 Reality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 握手服务器地址和拨号字段（必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<RealityHandshake>,

    /// 私钥（必填）
    /// 通过 `sing-box generate reality-keypair` 生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,

    /// 短ID 列表（必填）
    /// 零到八位的十六进制字符串
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_id: Option<Vec<String>>,

    /// 服务器和客户端之间的最大时间差
    /// 为空则禁用检查
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_time_difference: Option<Duration>,
}

/// Reality 出站配置（客户端）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RealityOutboundConfig {
    /// 启用 Reality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 公钥（必填）
    /// 通过 `sing-box generate reality-keypair` 生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,

    /// 短 ID（必填）
    /// 零到八位的十六进制字符串
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_id: Option<String>,
}

/// Reality 握手配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RealityHandshake {
    /// 握手服务器地址
    pub server: String,

    /// 握手服务器端口
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,

    // 以下为拨号字段（Dial Fields）
    /// 绑定的网络接口
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_interface: Option<String>,

    /// 绑定的 IPv4 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_bind_address: Option<String>,

    /// 绑定的 IPv6 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_bind_address: Option<String>,

    /// 路由标记（仅限 Linux）
    /// 支持整数（如 1234）和十六进制字符串（如 "0x1234"）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<RoutingMark>,

    /// 重用地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_addr: Option<bool>,

    /// 网络命名空间（仅限 Linux）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netns: Option<String>,

    /// 连接超时
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<Duration>,

    /// 启用 TCP 快速打开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_fast_open: Option<bool>,

    /// 启用 TCP 多路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_multi_path: Option<bool>,

    /// 启用 UDP 分片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_fragment: Option<bool>,

    /// 域名解析策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_strategy: Option<DomainStrategy>,

    /// 回退延迟
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_delay: Option<Duration>,
}

// ============================================================================
// ACME 配置
// ============================================================================

/// ACME 自动证书配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AcmeConfig {
    ///域名列表
    /// 如果为空则禁用 ACME
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,

    /// 存储 ACME 数据的目录
    /// 如果为空则使用 `$XDG_DATA_HOME/certmagic|$HOME/.local/share/certmagic`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_directory: Option<String>,

    /// 当ClientHello 的 ServerName 字段为空时使用的服务器名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_server_name: Option<String>,

    /// 创建或选择现有 ACME 服务器账户时使用的电子邮件地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// ACME CA 提供商
    /// 可选值: letsencrypt（默认）、zerossl、自定义 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AcmeProvider>,

    /// 禁用所有 HTTP挑战
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_http_challenge: Option<bool>,

    /// 禁用所有 TLS-ALPN 挑战
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_tls_alpn_challenge: Option<bool>,

    /// ACME HTTP 挑战的备用端口
    /// 如果非空，将使用此端口而不是 80
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_http_port: Option<u16>,

    /// ACME TLS-ALPN 挑战的备用端口
    /// 系统必须将 443 转发到此端口才能成功挑战
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_tls_port: Option<u16>,

    /// 外部账户绑定（EAB）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<AcmeExternalAccount>,

    /// ACME DNS01 挑战字段
    /// 如果配置，其他挑战方法将被禁用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns01_challenge: Option<serde_json::Value>,
}

/// ACME 外部账户绑定
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AcmeExternalAccount {
    /// 密钥标识符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,

    /// MAC 密钥
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_key: Option<String>,
}

// ============================================================================
// uTLS 配置
// ============================================================================

/// uTLS 配置
/// uTLS 是 "crypto/tls" 的分支，提供 ClientHello 指纹伪装能力
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UtlsConfig {
    /// 启用 uTLS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 指纹类型
    /// 如果为空则使用 chrome 指纹
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<UtlsFingerprint>,
}

// ============================================================================
// 枚举类型
// ============================================================================

/// TLS 版本
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TlsVersion {
    #[serde(rename = "1.0")]
    Tls10,
    #[serde(rename = "1.1")]
    Tls11,
    #[serde(rename = "1.2")]
    Tls12,
    #[serde(rename = "1.3")]
    Tls13,
}

/// TLS 密码套件
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CipherSuite {
    #[serde(rename = "TLS_RSA_WITH_AES_128_CBC_SHA")]
    TlsRsaWithAes128CbcSha,
    #[serde(rename = "TLS_RSA_WITH_AES_256_CBC_SHA")]
    TlsRsaWithAes256CbcSha,
    #[serde(rename = "TLS_RSA_WITH_AES_128_GCM_SHA256")]
    TlsRsaWithAes128GcmSha256,
    #[serde(rename = "TLS_RSA_WITH_AES_256_GCM_SHA384")]
    TlsRsaWithAes256GcmSha384,
    #[serde(rename = "TLS_AES_128_GCM_SHA256")]
    TlsAes128GcmSha256,
    #[serde(rename = "TLS_AES_256_GCM_SHA384")]
    TlsAes256GcmSha384,
    #[serde(rename = "TLS_CHACHA20_POLY1305_SHA256")]
    TlsChacha20Poly1305Sha256,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA")]
    TlsEcdheEcdsaWithAes128CbcSha,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA")]
    TlsEcdheEcdsaWithAes256CbcSha,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA")]
    TlsEcdheRsaWithAes128CbcSha,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA")]
    TlsEcdheRsaWithAes256CbcSha,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256")]
    TlsEcdheEcdsaWithAes128GcmSha256,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384")]
    TlsEcdheEcdsaWithAes256GcmSha384,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256")]
    TlsEcdheRsaWithAes128GcmSha256,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384")]
    TlsEcdheRsaWithAes256GcmSha384,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256")]
    TlsEcdheEcdsaWithChacha20Poly1305Sha256,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256")]
    TlsEcdheRsaWithChacha20Poly1305Sha256,
}

/// 密钥交换机制（曲线偏好）
/// 自 sing-box 1.13.0 起可用
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CurvePreference {
    P256,
    P384,
    P521,
    X25519,
    #[serde(rename = "X25519MLKEM768")]
    X25519Mlkem768,
}

/// 客户端认证类型
/// 自 sing-box 1.13.0 起可用
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClientAuthentication {
    /// 不要求客户端证书（默认）
    No,
    /// 请求客户端证书但不要求
    Request,
    /// 要求任何客户端证书
    RequireAny,
    /// 如果提供则验证客户端证书
    VerifyIfGiven,
    /// 要求并验证客户端证书
    RequireAndVerify,
}

/// ACME 提供商
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AcmeProvider {
    /// 预定义提供商
    Preset(AcmeProviderPreset),
    /// 自定义 URL
    Custom(String),
}

/// 预定义 ACME 提供商
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AcmeProviderPreset {
    /// Let's Encrypt（默认）
    LetsEncrypt,
    /// ZeroSSL
    ZeroSSL,
}

/// uTLS 指纹类型
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UtlsFingerprint {
    Chrome,
    Firefox,
    Edge,
    Safari,
    #[serde(rename = "360")]
    Browser360,
    #[serde(rename = "qq")]
    QQ,
    #[serde(rename = "ios")]
    IOS,
    Android,
    Random,
    Randomized,
}

// ============================================================================
// Default 实现
// ============================================================================

impl Default for InboundTlsConfig {
    fn default() -> Self {
        Self {
            enabled: None,
            server_name: None,
            alpn: None,
            min_version: None,
            max_version: None,
            cipher_suites: None,
            curve_preferences: None,
            certificate: None,
            certificate_path: None,
            client_authentication: None,
            client_certificate: None,
            client_certificate_path: None,
            client_certificate_public_key_sha256: None,
            key: None,
            key_path: None,
            kernel_tx: None,
            kernel_rx: None,
            acme: None,
            ech: None,
            reality: None,
        }
    }
}

impl Default for OutboundTlsConfig {
    fn default() -> Self {
        Self {
            enabled: None,
            disable_sni: None,
            server_name: None,
            insecure: None,
            alpn: None,
            min_version: None,
            max_version: None,
            cipher_suites: None,
            curve_preferences: None,
            certificate: None,
            certificate_path: None,
            certificate_public_key_sha256: None,
            client_certificate: None,
            client_certificate_path: None,
            client_key: None,
            client_key_path: None,
            fragment: None,
            fragment_fallback_delay: None,
            record_fragment: None,
            ech: None,
            utls: None,
            reality: None,
        }
    }
}
