use serde::{Deserialize, Serialize};

//============================================================================
// DNS01 Challenge 配置
// 文档: https://sing-box.sagernet.org/configuration/shared/dns01_challenge/
//============================================================================

/// DNS01 Challenge 配置
/// 用于 ACME DNS-01 验证
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "provider", rename_all = "lowercase")]
pub enum Dns01Challenge {
    /// 阿里云 DNS
    #[serde(rename = "alidns")]
    AliDns(AliDnsConfig),

    /// Cloudflare DNSCloudflare(CloudflareConfig),
}

//============================================================================
// 阿里云 DNS 配置
//============================================================================

/// 阿里云 DNS 配置
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AliDnsConfig {
    /// Access Key ID
    pub access_key_id: String,

    /// Access Key Secret
    pub access_key_secret: String,

    /// 区域 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

//============================================================================
// Cloudflare 配置
//============================================================================

/// Cloudflare DNS 配置
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudflareConfig {
    /// API Token
    pub api_token: String,
}

//============================================================================
// 构建器方法
//============================================================================

impl Dns01Challenge {
    /// 创建阿里云 DNS 配置
    pub fn alidns(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
    ) -> Self {
        Dns01Challenge::AliDns(AliDnsConfig {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            region_id: None,
        })
    }

    /// 创建阿里云 DNS 配置（带区域）
    pub fn alidns_with_region(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
        region_id: impl Into<String>,
    ) -> Self {
        Dns01Challenge::AliDns(AliDnsConfig {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            region_id: Some(region_id.into()),
        })
    }

    /// 创建 Cloudflare DNS 配置
    pub fn cloudflare(api_token: impl Into<String>) -> Self {
        Dns01Challenge::Cloudflare(CloudflareConfig {
            api_token: api_token.into(),
        })
    }
}

impl AliDnsConfig {
    /// 创建新的阿里云 DNS 配置
    pub fn new(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
    ) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            region_id: None,
        }
    }

    /// 设置区域 ID
    pub fn with_region(mut self, region_id: impl Into<String>) -> Self {
        self.region_id = Some(region_id.into());
        self
    }
}

impl CloudflareConfig {
    /// 创建新的 Cloudflare DNS 配置
    pub fn new(api_token: impl Into<String>) -> Self {
        Self {
            api_token: api_token.into(),
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
    fn test_alidns_serialize() {
        let challenge = Dns01Challenge::alidns("my_access_key_id", "my_access_key_secret");

        let json = serde_json::to_string(&challenge).unwrap();
        assert!(json.contains("\"provider\":\"alidns\""));
        assert!(json.contains("\"access_key_id\":\"my_access_key_id\""));
        assert!(json.contains("\"access_key_secret\":\"my_access_key_secret\""));
    }

    #[test]
    fn test_alidns_with_region_serialize() {
        let challenge =
            Dns01Challenge::alidns_with_region("key_id", "key_secret", "cn-hangzhou");

        let json = serde_json::to_string(&challenge).unwrap();
        assert!(json.contains("\"provider\":\"alidns\""));
        assert!(json.contains("\"region_id\":\"cn-hangzhou\""));
    }

    #[test]
    fn test_alidns_deserialize() {
        let json = r#"{
            "provider": "alidns",
            "access_key_id": "my_key_id",
            "access_key_secret": "my_key_secret",
            "region_id": "cn-beijing"
        }"#;

        let challenge: Dns01Challenge = serde_json::from_str(json).unwrap();
        if let Dns01Challenge::AliDns(config) = challenge {
            assert_eq!(config.access_key_id, "my_key_id");
            assert_eq!(config.access_key_secret, "my_key_secret");
            assert_eq!(config.region_id, Some("cn-beijing".to_string()));
        } else {
            panic!("Expected AliDns");
        }
    }

    #[test]
    fn test_cloudflare_serialize() {
        let challenge = Dns01Challenge::cloudflare("my_api_token");

        let json = serde_json::to_string(&challenge).unwrap();
        assert!(json.contains("\"provider\":\"cloudflare\""));
        assert!(json.contains("\"api_token\":\"my_api_token\""));
    }

    #[test]
    fn test_cloudflare_deserialize() {
        let json = r#"{
            "provider": "cloudflare",
            "api_token": "my_cloudflare_token"
        }"#;

        let challenge: Dns01Challenge = serde_json::from_str(json).unwrap();
        if let Dns01Challenge::Cloudflare(config) = challenge {
            assert_eq!(config.api_token, "my_cloudflare_token");
        } else {
            panic!("Expected Cloudflare");
        }
    }

    #[test]
    fn test_alidns_config_builder() {
        let config = AliDnsConfig::new("key_id", "key_secret").with_region("cn-shanghai");

        assert_eq!(config.access_key_id, "key_id");
        assert_eq!(config.access_key_secret, "key_secret");
        assert_eq!(config.region_id, Some("cn-shanghai".to_string()));
    }

    #[test]
    fn test_cloudflare_config_new() {
        let config = CloudflareConfig::new("token123");

        assert_eq!(config.api_token, "token123");
    }
}
