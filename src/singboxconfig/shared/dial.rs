use serde::{Deserialize, Serialize};

use crate::singboxconfig::types::{
    DomainStrategy, Duration, NetworkStrategy, NetworkType, RoutingMark,
};

//============================================================================
// 拨号字段配置
// ============================================================================

/// 拨号字段配置
/// 用于出站连接的通用配置
/// 文档: https://sing-box.sagernet.org/configuration/shared/dial/
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DialFields {
    /// 上游出站的标签
    /// 如果启用，所有其他字段将被忽略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,

    /// 要绑定的网络接口
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_interface: Option<String>,

    /// 要绑定的 IPv4 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_bind_address: Option<String>,

    /// 要绑定的 IPv6 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_bind_address: Option<String>,

    /// 设置 netfilter 路由标记（仅限 Linux）
    /// 支持整数（如 1234）和十六进制字符串（如 "0x1234"）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<RoutingMark>,

    /// 重用监听地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_addr: Option<bool>,

    /// 设置网络命名空间，名称或路径（仅限 Linux）
    /// 自sing-box 1.12.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netns: Option<String>,

    /// 连接超时
    /// 格式为 golang的 Duration 格式，如 "300ms", "1.5h", "2h45m"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<Duration>,

    /// 启用 TCP 快速打开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_fast_open: Option<bool>,

    /// 启用 TCP 多路径（需要 Go 1.21）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_multi_path: Option<bool>,

    /// 禁用 TCP 保活
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_tcp_keep_alive: Option<bool>,

    /// TCP 保活初始周期
    /// 默认值: 5m
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_keep_alive: Option<Duration>,

    /// TCP 保活间隔
    /// 默认值: 75s
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_keep_alive_interval: Option<Duration>,

    /// 启用 UDP 分片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_fragment: Option<bool>,

    /// 域名解析器
    /// 用于解析域名的DNS 解析器
    /// 可以是字符串（服务器标签）或对象（完整配置）
    /// 自 sing-box 1.12.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_resolver: Option<DomainResolver>,

    /// 网络策略
    /// 用于选择网络接口的策略
    /// 自sing-box 1.11.0 起可用
    /// 仅在 Android 和 Apple 平台的图形客户端中支持
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_strategy: Option<NetworkStrategy>,

    /// 网络类型
    /// 使用 default 或 hybrid 策略时要使用的网络类型
    /// 或使用 fallback 策略时的首选网络类型
    /// 自 sing-box 1.11.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<Vec<NetworkType>>,

    /// 回退网络类型
    /// 使用 fallback 策略时，首选网络不可用或超时时的回退网络类型
    /// 自 sing-box 1.11.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_network_type: Option<Vec<NetworkType>>,

    /// 回退延迟
    /// 在生成 RFC 6555 快速回退连接之前等待的时间
    /// 默认值: 300ms
    /// 自 sing-box 1.11.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_delay: Option<Duration>,

    /// 域名解析策略
    /// 自 sing-box 1.12.0 起已弃用，将在 1.14.0 中移除
    #[deprecated(since = "1.12.0", note = "请使用 domain_resolver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_strategy: Option<DomainStrategy>,
}

// ============================================================================
// 域名解析器
// ============================================================================

/// 域名解析器配置
/// 可以是简单的服务器标签字符串，或完整的配置对象
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DomainResolver {
    /// 简单的服务器标签
    Tag(String),
    /// 完整的配置对象
    Config(DomainResolverConfig),
}

/// 域名解析器完整配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainResolverConfig {
    /// DNS 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    ///禁用缓存
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,

    /// 重写 TTL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite_ttl: Option<u32>,

    /// 客户端子网
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
}

impl From<String> for DomainResolver {
    fn from(s: String) -> Self {
        DomainResolver::Tag(s)
    }
}

impl From<&str> for DomainResolver {
    fn from(s: &str) -> Self {
        DomainResolver::Tag(s.to_string())
    }
}

impl Default for DomainResolver {
    fn default() -> Self {
        DomainResolver::Tag(String::new())
    }
}
