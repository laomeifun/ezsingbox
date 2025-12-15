use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use rand::RngCore;
use std::net::IpAddr;
use std::time::Duration as StdDuration;

//============================================================================
// 公网 IP 获取
//============================================================================

/// 公网 IP 获取错误
#[derive(Debug, Clone)]
pub enum PublicIpError {
    /// 网络请求失败
    NetworkError(String),
    /// 解析 IP 失败
    ParseError(String),
    /// 所有服务都不可用
    AllServicesFailed,
}

impl std::fmt::Display for PublicIpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PublicIpError::NetworkError(msg) => write!(f, "网络请求失败: {}", msg),
            PublicIpError::ParseError(msg) => write!(f, "解析 IP 失败: {}", msg),
            PublicIpError::AllServicesFailed => write!(f, "所有公网 IP 服务都不可用"),
        }
    }
}

impl std::error::Error for PublicIpError {}

/// 获取公网 IP 的服务列表
const PUBLIC_IP_SERVICES: &[&str] = &[
    "https://api.ipify.org",
    "https://ifconfig.me/ip",
    "https://icanhazip.com",
    "https://ip.sslip.io",
    "https://api.ip.sb/ip",
];

/// 获取公网 IP
/// 依次尝试多个服务，直到成功获取
pub fn get_public_ip() -> Result<IpAddr, PublicIpError> {
    get_public_ip_with_timeout(StdDuration::from_secs(5))
}

/// 获取公网 IP（指定超时时间）
pub fn get_public_ip_with_timeout(timeout: StdDuration) -> Result<IpAddr, PublicIpError> {
    for service in PUBLIC_IP_SERVICES {
        match try_get_ip_from_service(service, timeout) {
            Ok(ip) => return Ok(ip),
            Err(_) => continue,
        }
    }
    Err(PublicIpError::AllServicesFailed)
}

/// 从指定服务获取 IP
fn try_get_ip_from_service(url: &str, timeout: StdDuration) -> Result<IpAddr, PublicIpError> {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(timeout))
        .build();
    let agent = ureq::Agent::new_with_config(config);

    let response = agent
        .get(url)
        .call()
        .map_err(|e| PublicIpError::NetworkError(e.to_string()))?;

    let body = response
        .into_body()
        .read_to_string()
        .map_err(|e| PublicIpError::NetworkError(e.to_string()))?;

    let ip_str = body.trim();
    ip_str
        .parse::<IpAddr>()
        .map_err(|e| PublicIpError::ParseError(format!("{}: {}", ip_str, e)))
}

//============================================================================
// 密码生成
//============================================================================

/// 生成随机密码（16 字节 base64 编码）
pub fn generate_password() -> String {
    generate_password_with_length(16)
}

/// 生成指定长度的随机密码（base64 编码）
pub fn generate_password_with_length(length: usize) -> String {
    let mut bytes = vec![0u8; length];
    rand::rng().fill_bytes(&mut bytes);
    BASE64.encode(&bytes)
}

/// 生成随机字节数组
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; length];
    rand::rng().fill_bytes(&mut bytes);
    bytes
}

/// 生成十六进制字符串
pub fn generate_hex_string(length: usize) -> String {
    let bytes = generate_random_bytes(length);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

//============================================================================
// 域名生成
//============================================================================

/// 生成 sslip.io 域名
/// 例如：1.2.3.4 -> 1-2-3-4.sslip.io
pub fn generate_sslip_domain(ip: &IpAddr) -> String {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            format!(
                "{}-{}-{}-{}.sslip.io",
                octets[0], octets[1], octets[2], octets[3]
            )
        }
        IpAddr::V6(ipv6) => {
            // IPv6 使用 -- 分隔
            let segments: Vec<String> =
                ipv6.segments().iter().map(|s| format!("{:x}", s)).collect();
            format!("{}.sslip.io", segments.join("-"))
        }
    }
}

/// 生成 nip.io 域名（另一个泛解析服务）
/// 例如：1.2.3.4 -> 1.2.3.4.nip.io
pub fn generate_nip_domain(ip: &IpAddr) -> String {
    format!("{}.nip.io", ip)
}

//============================================================================
// UUID 生成
//============================================================================

/// 生成 UUID v4
pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// 生成不带连字符的 UUID
pub fn generate_uuid_simple() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

//============================================================================
// 用户配置
//============================================================================

/// 通用用户配置
/// 如果只提供 name，则自动生成 password
#[derive(Debug, Clone)]
pub struct UserConfig {
    /// 用户名
    pub name: String,
    /// 用户密码（可选，不提供则自动生成）
    pub password: Option<String>,
}

impl UserConfig {
    /// 创建新用户配置（自动生成密码）
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            password: None,
        }
    }

    /// 创建带密码的用户配置
    pub fn with_password(name: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            password: Some(password.into()),
        }
    }

    /// 获取密码（如果未设置则生成）
    pub fn get_or_generate_password(&self) -> String {
        self.password.clone().unwrap_or_else(generate_password)
    }
}

//============================================================================
// TLS 配置模式
//============================================================================

/// TLS 配置模式
#[derive(Debug, Clone)]
pub enum TlsMode {
    /// 使用 ACME 自动获取证书（Let's Encrypt）
    Acme {
        /// 域名（如果不提供，使用 sslip.io）
        domain: Option<String>,
        /// ACME 邮箱（可选）
        email: Option<String>,
    },
    /// 使用自定义证书
    Custom {
        /// 证书路径
        certificate_path: String,
        /// 私钥路径
        key_path: String,
        /// 服务器名称
        server_name: Option<String>,
    },
    /// 禁用 TLS（不推荐）
    Disabled,
}

impl Default for TlsMode {
    fn default() -> Self {
        TlsMode::Acme {
            domain: None,
            email: None,
        }
    }
}

impl TlsMode {
    /// 创建 ACME 模式（使用 sslip.io）
    pub fn acme() -> Self {
        Self::default()
    }

    /// 创建 ACME 模式（指定域名）
    pub fn acme_with_domain(domain: impl Into<String>) -> Self {
        TlsMode::Acme {
            domain: Some(domain.into()),
            email: None,
        }
    }

    /// 创建 ACME 模式（指定域名和邮箱）
    pub fn acme_with_domain_and_email(domain: impl Into<String>, email: impl Into<String>) -> Self {
        TlsMode::Acme {
            domain: Some(domain.into()),
            email: Some(email.into()),
        }
    }

    /// 创建自定义证书模式
    pub fn custom(certificate_path: impl Into<String>, key_path: impl Into<String>) -> Self {
        TlsMode::Custom {
            certificate_path: certificate_path.into(),
            key_path: key_path.into(),
            server_name: None,
        }
    }

    /// 创建自定义证书模式（带服务器名称）
    pub fn custom_with_server_name(
        certificate_path: impl Into<String>,
        key_path: impl Into<String>,
        server_name: impl Into<String>,
    ) -> Self {
        TlsMode::Custom {
            certificate_path: certificate_path.into(),
            key_path: key_path.into(),
            server_name: Some(server_name.into()),
        }
    }

    /// 禁用 TLS
    pub fn disabled() -> Self {
        TlsMode::Disabled
    }
}

//============================================================================
// 单元测试
//============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        let password = generate_password();
        assert!(!password.is_empty());
        // Base64 编码的16 字节应该是 24 字符（包含填充）
        assert!(password.len() >= 20);
    }

    #[test]
    fn test_generate_password_with_length() {
        let password = generate_password_with_length(32);
        assert!(!password.is_empty());
    }

    #[test]
    fn test_generate_hex_string() {
        let hex = generate_hex_string(8);
        assert_eq!(hex.len(), 16); // 8 bytes = 16 hex chars
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_sslip_domain_v4() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let domain = generate_sslip_domain(&ip);
        assert_eq!(domain, "1-2-3-4.sslip.io");
    }

    #[test]
    fn test_generate_sslip_domain_v6() {
        let ip: IpAddr = "2001:db8::1".parse().unwrap();
        let domain = generate_sslip_domain(&ip);
        assert!(domain.ends_with(".sslip.io"));
    }

    #[test]
    fn test_generate_nip_domain() {
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        let domain = generate_nip_domain(&ip);
        assert_eq!(domain, "1.2.3.4.nip.io");
    }

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert_eq!(uuid.len(), 36); // UUID with hyphens
        assert!(uuid.contains('-'));
    }

    #[test]
    fn test_generate_uuid_simple() {
        let uuid = generate_uuid_simple();
        assert_eq!(uuid.len(), 32); // UUID without hyphens
        assert!(!uuid.contains('-'));
    }

    #[test]
    fn test_user_config_new() {
        let user = UserConfig::new("test_user");
        assert_eq!(user.name, "test_user");
        assert!(user.password.is_none());
    }

    #[test]
    fn test_user_config_with_password() {
        let user = UserConfig::with_password("test_user", "my_password");
        assert_eq!(user.name, "test_user");
        assert_eq!(user.password, Some("my_password".to_string()));
    }

    #[test]
    fn test_user_config_get_or_generate_password() {
        let user1 = UserConfig::new("user1");
        let password1 = user1.get_or_generate_password();
        assert!(!password1.is_empty());

        let user2 = UserConfig::with_password("user2", "fixed_password");
        let password2 = user2.get_or_generate_password();
        assert_eq!(password2, "fixed_password");
    }

    #[test]
    fn test_tls_mode_default() {
        let mode = TlsMode::default();
        assert!(matches!(
            mode,
            TlsMode::Acme {
                domain: None,
                email: None
            }
        ));
    }

    #[test]
    fn test_tls_mode_acme_with_domain() {
        let mode = TlsMode::acme_with_domain("example.com");
        if let TlsMode::Acme { domain, email } = mode {
            assert_eq!(domain, Some("example.com".to_string()));
            assert!(email.is_none());
        } else {
            panic!("Expected Acme mode");
        }
    }

    #[test]
    fn test_tls_mode_custom() {
        let mode = TlsMode::custom("/path/to/cert.pem", "/path/to/key.pem");
        if let TlsMode::Custom {
            certificate_path,
            key_path,
            server_name,
        } = mode
        {
            assert_eq!(certificate_path, "/path/to/cert.pem");
            assert_eq!(key_path, "/path/to/key.pem");
            assert!(server_name.is_none());
        } else {
            panic!("Expected Custom mode");
        }
    }
}
