use serde::{Deserialize, Serialize};

//============================================================================
// 网络策略
// ============================================================================

/// 网络策略
/// 用于选择网络接口的策略
/// 自sing-box 1.11.0 起可用
///仅在 Android 和 Apple 平台的图形客户端中支持，需启用 `auto_detect_interface`
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkStrategy {
    /// 默认策略
    /// 按顺序连接到默认网络或 network_type 中指定的网络
    Default,

    /// 混合策略
    /// 同时连接到所有网络或 network_type 中指定的网络
    Hybrid,

    /// 回退策略
    /// 同时连接到默认网络或首选网络，不可用或超时时尝试回退网络
    /// 当首选接口失败或超时时，将进入 15 秒快速回退状态
    Fallback,
}

impl Default for NetworkStrategy {
    fn default() -> Self {
        NetworkStrategy::Default
    }
}

impl std::fmt::Display for NetworkStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkStrategy::Default => write!(f, "default"),
            NetworkStrategy::Hybrid => write!(f, "hybrid"),
            NetworkStrategy::Fallback => write!(f, "fallback"),
        }
    }
}

// ============================================================================
// 网络类型
// ============================================================================

/// 网络类型
/// 用于指定要使用的网络类型
/// 自 sing-box 1.11.0 起可用
/// 仅在 Android 和 Apple 平台的图形客户端中支持，需启用 `auto_detect_interface`
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkType {
    /// WiFi 网络
    Wifi,

    /// 蜂窝网络（移动数据）
    Cellular,

    /// 以太网
    Ethernet,

    /// 其他网络类型
    Other,
}

impl std::fmt::Display for NetworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkType::Wifi => write!(f, "wifi"),
            NetworkType::Cellular => write!(f, "cellular"),
            NetworkType::Ethernet => write!(f, "ethernet"),
            NetworkType::Other => write!(f, "other"),
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
    fn test_network_strategy_serialize() {
        let strategy = NetworkStrategy::Default;
        let json = serde_json::to_string(&strategy).unwrap();
        assert_eq!(json, "\"default\"");

        let strategy = NetworkStrategy::Hybrid;
        let json = serde_json::to_string(&strategy).unwrap();
        assert_eq!(json, "\"hybrid\"");

        let strategy = NetworkStrategy::Fallback;
        let json = serde_json::to_string(&strategy).unwrap();
        assert_eq!(json, "\"fallback\"");
    }

    #[test]
    fn test_network_strategy_deserialize() {
        let strategy: NetworkStrategy = serde_json::from_str("\"default\"").unwrap();
        assert_eq!(strategy, NetworkStrategy::Default);

        let strategy: NetworkStrategy = serde_json::from_str("\"hybrid\"").unwrap();
        assert_eq!(strategy, NetworkStrategy::Hybrid);

        let strategy: NetworkStrategy = serde_json::from_str("\"fallback\"").unwrap();
        assert_eq!(strategy, NetworkStrategy::Fallback);
    }

    #[test]
    fn test_network_type_serialize() {
        let net_type = NetworkType::Wifi;
        let json = serde_json::to_string(&net_type).unwrap();
        assert_eq!(json, "\"wifi\"");

        let net_type = NetworkType::Cellular;
        let json = serde_json::to_string(&net_type).unwrap();
        assert_eq!(json, "\"cellular\"");
    }

    #[test]
    fn test_network_type_deserialize() {
        let net_type: NetworkType = serde_json::from_str("\"wifi\"").unwrap();
        assert_eq!(net_type, NetworkType::Wifi);

        let net_type: NetworkType = serde_json::from_str("\"ethernet\"").unwrap();
        assert_eq!(net_type, NetworkType::Ethernet);
    }

    #[test]
    fn test_display() {
        assert_eq!(NetworkStrategy::Default.to_string(), "default");
        assert_eq!(NetworkStrategy::Hybrid.to_string(), "hybrid");
        assert_eq!(NetworkType::Wifi.to_string(), "wifi");
        assert_eq!(NetworkType::Cellular.to_string(), "cellular");
    }
}
