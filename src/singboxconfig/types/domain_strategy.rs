use serde::{Deserialize, Serialize};

//============================================================================
// 域名解析策略
// ============================================================================

/// 域名解析策略
/// 用于指定如何解析域名到IP 地址
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainStrategy {
    /// 优先使用 IPv4
    /// 如果有 IPv4 地址则使用，否则使用 IPv6
    PreferIpv4,

    /// 优先使用 IPv6
    /// 如果有 IPv6 地址则使用，否则使用 IPv4
    PreferIpv6,

    /// 仅使用 IPv4
    /// 只解析 A 记录
    Ipv4Only,

    /// 仅使用 IPv6
    /// 只解析 AAAA 记录
    Ipv6Only,
}

impl Default for DomainStrategy {
    fn default() -> Self {
        DomainStrategy::PreferIpv4
    }
}

impl std::fmt::Display for DomainStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainStrategy::PreferIpv4 => write!(f, "prefer_ipv4"),
            DomainStrategy::PreferIpv6 => write!(f, "prefer_ipv6"),
            DomainStrategy::Ipv4Only => write!(f, "ipv4_only"),
            DomainStrategy::Ipv6Only => write!(f, "ipv6_only"),
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
    fn test_serialize() {
        let strategy = DomainStrategy::PreferIpv4;
        let json = serde_json::to_string(&strategy).unwrap();
        assert_eq!(json, "\"prefer_ipv4\"");

        let strategy = DomainStrategy::Ipv6Only;
        let json = serde_json::to_string(&strategy).unwrap();
        assert_eq!(json, "\"ipv6_only\"");
    }

    #[test]
    fn test_deserialize() {
        let strategy: DomainStrategy = serde_json::from_str("\"prefer_ipv6\"").unwrap();
        assert_eq!(strategy, DomainStrategy::PreferIpv6);

        let strategy: DomainStrategy = serde_json::from_str("\"ipv4_only\"").unwrap();
        assert_eq!(strategy, DomainStrategy::Ipv4Only);
    }

    #[test]
    fn test_display() {
        assert_eq!(DomainStrategy::PreferIpv4.to_string(), "prefer_ipv4");
        assert_eq!(DomainStrategy::PreferIpv6.to_string(), "prefer_ipv6");
        assert_eq!(DomainStrategy::Ipv4Only.to_string(), "ipv4_only");
        assert_eq!(DomainStrategy::Ipv6Only.to_string(), "ipv6_only");
    }
}
