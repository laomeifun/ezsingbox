//! sing-box DNS 配置类型定义
//!
//! 此模块包含 sing-box DNS 配置的完整类型定义
//! 参考文档: https://sing-box.sagernet.org/configuration/dns/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::singboxconfig::types::{DomainStrategy, StringOrArray};

// ============================================================================
// DNS 主配置
// ============================================================================

/// DNS 配置
///
/// # 示例
///
/// ```json
/// {
///   "dns": {
///     "servers": [],
///     "rules": [],
///     "final": "local",
///     "strategy": "prefer_ipv4",
///     "disable_cache": false,
///     "disable_expire": false,
///     "independent_cache": false,
///     "cache_capacity": 0,
///     "reverse_mapping": false,
///     "client_subnet": "",
///     "fakeip": {}
///   }
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Dns {
    /// DNS 服务器列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<DnsServer>>,

    /// DNS 规则列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<DnsRule>>,

    /// 默认 DNS 服务器标签
    /// 如果为空，则使用第一个服务器
    #[serde(rename = "final", skip_serializing_if = "Option::is_none")]
    pub final_server: Option<String>,

    /// 默认域名解析策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<DomainStrategy>,

    /// 禁用DNS 缓存
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,

    /// 禁用 DNS 缓存过期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_expire: Option<bool>,

    /// 使每个 DNS 服务器的缓存独立
    /// 启用后会略微降低性能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub independent_cache: Option<bool>,

    /// LRU 缓存容量
    /// 小于 1024 的值将被忽略
    /// Since sing-box 1.11.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_capacity: Option<u32>,

    /// 存储 IP 地址的反向映射
    /// 用于在路由时提供域名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_mapping: Option<bool>,

    /// 默认 EDNS Client Subnet
    /// Since sing-box 1.9.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,

    /// FakeIP 配置 (Legacy, deprecated in 1.12.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fakeip: Option<LegacyFakeIP>,
}

// ============================================================================
// DNS Server 类型
// ============================================================================

/// DNS 服务器配置
///
/// 使用 `type` 字段区分不同类型的 DNS 服务器
/// Since sing-box 1.12.0 引入新的类型化服务器配置
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DnsServer {
    /// 本地 DNS 服务器
    Local(LocalDnsServer),

    /// Hosts 文件 DNS 服务器
    Hosts(HostsDnsServer),

    /// TCP DNS 服务器
    Tcp(RemoteDnsServer),

    /// UDP DNS 服务器
    Udp(RemoteDnsServer),

    /// DNS over TLS 服务器
    Tls(RemoteDnsServer),

    /// DNS over QUIC 服务器
    Quic(RemoteDnsServer),

    /// DNS over HTTPS 服务器
    Https(RemoteDnsServer),

    /// DNS over HTTP/3 服务器
    H3(RemoteDnsServer),

    /// DHCP DNS 服务器
    Dhcp(DhcpDnsServer),

    /// FakeIP DNS 服务器
    #[serde(rename = "fakeip")]
    FakeIP(FakeIPDnsServer),

    /// Tailscale DNS 服务器
    Tailscale(TailscaleDnsServer),

    /// Resolved DNS 服务器
    Resolved(ResolvedDnsServer),

    /// Legacy DNS 服务器配置 (空类型或未指定类型)
    #[serde(other)]
    Legacy,
}

/// Legacy DNS 服务器配置
/// 用于兼容旧版本配置格式
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct LegacyDnsServer {
    /// 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// 服务器地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// 地址解析器
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_resolver: Option<String>,

    /// 地址解析策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_strategy: Option<DomainStrategy>,

    /// 地址回退延迟
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_fallback_delay: Option<String>,

    /// 域名解析策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<DomainStrategy>,

    /// 出站代理
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,

    /// Client Subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
}

/// 本地 DNS 服务器
/// Since sing-box 1.12.0
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct LocalDnsServer {
    /// 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// 优先使用 Go 原生解析
    /// Since sing-box 1.13.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_go: Option<bool>,

    /// 拨号字段
    #[serde(flatten)]
    pub dial: Option<DnsDialFields>,
}

/// Hosts DNS 服务器
/// Since sing-box 1.12.0
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct HostsDnsServer {
    /// 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Hosts 文件路径列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<StringOrArray>,

    /// 预定义的 hosts 映射
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined: Option<HashMap<String, StringOrArray>>,
}

///远程 DNS 服务器 (TCP/UDP/TLS/QUIC/HTTPS/H3)
/// Since sing-box 1.12.0
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct RemoteDnsServer {
    /// 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// 服务器地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// 服务器端口
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,

    /// 地址解析器
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_resolver: Option<String>,

    /// 地址解析策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_strategy: Option<DomainStrategy>,

    /// 域名解析策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<DomainStrategy>,

    /// Client Subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,

    /// 拨号字段
    #[serde(flatten)]
    pub dial: Option<DnsDialFields>,
}

/// DHCP DNS 服务器
/// Since sing-box 1.12.0
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct DhcpDnsServer {
    /// 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    ///监听的网络接口名称
    #[serde(rename = "interface", skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,

    /// 拨号字段
    #[serde(flatten)]
    pub dial: Option<DnsDialFields>,
}

/// FakeIP DNS 服务器
/// Since sing-box 1.12.0
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct FakeIPDnsServer {
    /// 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// IPv4 地址范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_range: Option<String>,

    /// IPv6 地址范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_range: Option<String>,
}

/// Tailscale DNS 服务器
/// Since sing-box 1.12.0
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct TailscaleDnsServer {
    /// 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Resolved DNS 服务器
/// Since sing-box 1.12.0
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct ResolvedDnsServer {
    /// 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// DNS拨号字段
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct DnsDialFields {
    /// 出站代理
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,

    /// 绑定接口
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_interface: Option<String>,

    /// 绑定地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_bind_address: Option<String>,

    /// 绑定地址 IPv6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_bind_address: Option<String>,

    /// 路由标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<u32>,

    /// 重用地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_addr: Option<bool>,

    /// 连接超时
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<String>,

    /// TCP 快速打开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_fast_open: Option<bool>,

    /// TCP 多路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_multi_path: Option<bool>,

    /// UDP 分片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_fragment: Option<bool>,
}

// ============================================================================
// Legacy FakeIP 配置
// ============================================================================

/// Legacy FakeIP 配置
/// Deprecated in sing-box 1.12.0
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct LegacyFakeIP {
    /// 启用 FakeIP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// IPv4 地址范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_range: Option<String>,

    /// IPv6 地址范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_range: Option<String>,
}

// ============================================================================
// DNS Rule 类型
// ============================================================================

/// DNS 规则
///
/// 支持两种类型：
/// - 默认规则：使用各种匹配条件
/// - 逻辑规则：使用 `and` 或 `or` 组合多个规则
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DnsRule {
    /// 逻辑规则Logical(LogicalDnsRule),
    /// 默认规则
    Default(DefaultDnsRule),
}

/// 默认 DNS 规则
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct DefaultDnsRule {
    //==========匹配条件 ==========
    /// 入站标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<StringOrArray>,

    /// IP 版本 (4 或 6)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<u8>,

    /// DNS 查询类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<QueryType>,

    /// 网络类型 (tcp或 udp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<StringOrArray>,

    /// 认证用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_user: Option<StringOrArray>,

    /// 嗅探协议
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<StringOrArray>,

    /// 完整域名匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<StringOrArray>,

    /// 域名后缀匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_suffix: Option<StringOrArray>,

    /// 域名关键字匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_keyword: Option<StringOrArray>,

    /// 域名正则匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_regex: Option<StringOrArray>,

    /// 源IP CIDR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_cidr: Option<StringOrArray>,

    /// 源 IP 是否为私有地址
    /// Since sing-box 1.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_is_private: Option<bool>,

    /// IP CIDR (用于地址过滤)
    /// Since sing-box 1.9.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_cidr: Option<StringOrArray>,

    /// IP 是否为私有地址 (用于地址过滤)
    /// Since sing-box 1.9.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_is_private: Option<bool>,

    /// 匹配任意 IP (用于地址过滤)
    /// Since sing-box 1.12.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_accept_any: Option<bool>,

    /// 源端口
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<PortOrArray>,

    /// 源端口范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<StringOrArray>,

    /// 目标端口
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<PortOrArray>,

    /// 目标端口范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<StringOrArray>,

    /// 进程名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<StringOrArray>,

    /// 进程路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path: Option<StringOrArray>,

    /// 进程路径正则
    /// Since sing-box 1.10.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path_regex: Option<StringOrArray>,

    /// Android 包名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<StringOrArray>,

    /// Linux 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<StringOrArray>,

    /// Linux 用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UidOrArray>,

    /// Clash 模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_mode: Option<String>,

    /// 网络类型 (wifi/cellular/ethernet/other)
    /// Since sing-box 1.11.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<StringOrArray>,

    /// 网络是否为计费网络
    /// Since sing-box 1.11.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_is_expensive: Option<bool>,

    /// 网络是否受限
    /// Since sing-box 1.11.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_is_constrained: Option<bool>,

    /// 接口地址
    /// Since sing-box 1.13.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_address: Option<HashMap<String, StringOrArray>>,

    /// 网络接口地址
    /// Since sing-box 1.13.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_address: Option<HashMap<String, StringOrArray>>,

    /// 默认接口地址
    /// Since sing-box 1.13.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_interface_address: Option<StringOrArray>,

    /// WiFi SSID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_ssid: Option<StringOrArray>,

    /// WiFi BSSID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_bssid: Option<StringOrArray>,

    /// 规则集
    /// Since sing-box 1.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<StringOrArray>,

    /// 规则集 IP CIDR 匹配源
    /// Since sing-box 1.10.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ip_cidr_match_source: Option<bool>,

    /// 规则集 IP CIDR 接受空结果
    /// Since sing-box 1.10.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ip_cidr_accept_empty: Option<bool>,

    /// 反转匹配结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,

    /// 出站匹配
    /// Deprecated in sing-box 1.12.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<StringOrArray>,

    // ========== 动作 ==========
    /// 规则动作
    /// Since sing-box 1.11.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<DnsRuleAction>,

    /// DNS 服务器标签
    /// Deprecated in sing-box 1.11.0, moved to action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// 禁用缓存
    /// Deprecated in sing-box 1.11.0, moved to action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,

    /// 重写TTL
    /// Deprecated in sing-box 1.11.0, moved to action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite_ttl: Option<u32>,

    /// Client Subnet
    /// Deprecated in sing-box 1.11.0, moved to action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,

    // ========== 废弃字段 ==========
    /// GeoSite (Deprecated in 1.8.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geosite: Option<StringOrArray>,

    /// 源 GeoIP (Deprecated in 1.8.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_geoip: Option<StringOrArray>,

    /// GeoIP (Deprecated in 1.8.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoip: Option<StringOrArray>,

    /// 规则集 IP CIDR 匹配源(旧名称)
    /// Deprecated in sing-box 1.10.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ipcidr_match_source: Option<bool>,
}

/// 逻辑DNS 规则
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct LogicalDnsRule {
    /// 规则类型，必须为 "logical"
    #[serde(rename = "type")]
    pub rule_type: String,

    ///逻辑模式 (and 或 or)
    pub mode: LogicalMode,

    /// 包含的规则
    pub rules: Vec<DefaultDnsRule>,

    /// 规则动作
    /// Since sing-box 1.11.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<DnsRuleAction>,

    /// DNS 服务器标签
    /// Deprecated in sing-box 1.11.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// 逻辑模式
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogicalMode {
    /// 所有条件都必须匹配
    And,
    /// 任一条件匹配即可
    Or,
}

impl Default for LogicalMode {
    fn default() -> Self {
        LogicalMode::And
    }
}

// ============================================================================
// DNS Rule Action 类型
// ============================================================================

/// DNS 规则动作
/// Since sing-box 1.11.0
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum DnsRuleAction {
    /// 路由到指定 DNS 服务器
    Route(DnsRouteAction),

    /// 路由到指定 DNS 服务器 (带选项)
    #[serde(rename = "route-options")]
    RouteOptions(DnsRouteOptionsAction),

    /// 拒绝请求
    Reject(DnsRejectAction),

    /// 预定义响应
    Predefined(DnsPredefinedAction),
}

/// DNS 路由动作
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct DnsRouteAction {
    /// DNS 服务器标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// 禁用缓存
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,

    /// 重写 TTL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite_ttl: Option<u32>,

    /// Client Subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
}

/// DNS 路由选项动作
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct DnsRouteOptionsAction {
    /// 禁用缓存
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,

    /// 重写 TTL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite_ttl: Option<u32>,

    /// Client Subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
}

/// DNS 拒绝动作
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct DnsRejectAction {
    /// 拒绝方法
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<DnsRejectMethod>,

    /// 不丢弃请求
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_drop: Option<bool>,
}

/// DNS 拒绝方法
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DnsRejectMethod {
    /// 返回默认响应
    Default,
    ///丢弃请求
    Drop,
}

/// DNS 预定义响应动作
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct DnsPredefinedAction {
    /// 预定义的RCode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcode: Option<String>,

    /// 预定义的响应记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<StringOrArray>,
}

// ============================================================================
//辅助类型
// ============================================================================

/// DNS 查询类型
/// 可以是整数或字符串类型名称
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum QueryType {
    /// 单个查询类型
    Single(QueryTypeValue),
    /// 多个查询类型
    Array(Vec<QueryTypeValue>),
}

/// 查询类型值
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum QueryTypeValue {
    /// 整数类型 (如 1, 28, 32768)
    Number(u16),
    /// 字符串类型名称 (如 "A", "AAAA", "HTTPS")
    Name(String),
}

/// 端口或端口数组
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PortOrArray {
    /// 单个端口
    Single(u16),
    /// 多个端口
    Array(Vec<u16>),
}

/// 用户ID 或用户 ID 数组
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UidOrArray {
    /// 单个用户 ID
    Single(u32),
    /// 多个用户 ID
    Array(Vec<u32>),
}

// ============================================================================
// 构建器方法
// ============================================================================

impl Dns {
    /// 创建新的 DNS 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加 DNS 服务器
    pub fn add_server(mut self, server: DnsServer) -> Self {
        self.servers.get_or_insert_with(Vec::new).push(server);
        self
    }

    /// 添加 DNS 规则
    pub fn add_rule(mut self, rule: DnsRule) -> Self {
        self.rules.get_or_insert_with(Vec::new).push(rule);
        self
    }

    /// 设置默认服务器
    pub fn final_server<S: Into<String>>(mut self, server: S) -> Self {
        self.final_server = Some(server.into());
        self
    }

    /// 设置域名解析策略
    pub fn strategy(mut self, strategy: DomainStrategy) -> Self {
        self.strategy = Some(strategy);
        self
    }

    /// 禁用缓存
    pub fn disable_cache(mut self, disable: bool) -> Self {
        self.disable_cache = Some(disable);
        self
    }

    /// 设置缓存容量
    pub fn cache_capacity(mut self, capacity: u32) -> Self {
        self.cache_capacity = Some(capacity);
        self
    }

    /// 启用反向映射
    pub fn reverse_mapping(mut self, enable: bool) -> Self {
        self.reverse_mapping = Some(enable);
        self
    }

    /// 设置 Client Subnet
    pub fn client_subnet<S: Into<String>>(mut self, subnet: S) -> Self {
        self.client_subnet = Some(subnet.into());
        self
    }
}

impl LocalDnsServer {
    /// 创建新的本地 DNS 服务器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置标签
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// 设置优先使用 Go 解析
    pub fn prefer_go(mut self, prefer: bool) -> Self {
        self.prefer_go = Some(prefer);
        self
    }
}

impl HostsDnsServer {
    /// 创建新的 Hosts DNS 服务器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置标签
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// 添加 hosts 文件路径
    pub fn path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = Some(StringOrArray::Single(path.into()));
        self
    }

    /// 添加多个 hosts 文件路径
    pub fn paths<I, S>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.path = Some(StringOrArray::Array(
            paths.into_iter().map(|s| s.into()).collect(),
        ));
        self
    }
}

impl RemoteDnsServer {
    /// 创建新的远程 DNS 服务器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置标签
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// 设置服务器地址
    pub fn server<S: Into<String>>(mut self, server: S) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置服务器端口
    pub fn server_port(mut self, port: u16) -> Self {
        self.server_port = Some(port);
        self
    }

    /// 设置地址解析器
    pub fn address_resolver<S: Into<String>>(mut self, resolver: S) -> Self {
        self.address_resolver = Some(resolver.into());
        self
    }

    /// 设置域名解析策略
    pub fn strategy(mut self, strategy: DomainStrategy) -> Self {
        self.strategy = Some(strategy);
        self
    }
}

impl FakeIPDnsServer {
    /// 创建新的 FakeIP DNS 服务器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置标签
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// 设置 IPv4 地址范围
    pub fn inet4_range<S: Into<String>>(mut self, range: S) -> Self {
        self.inet4_range = Some(range.into());
        self
    }

    /// 设置 IPv6 地址范围
    pub fn inet6_range<S: Into<String>>(mut self, range: S) -> Self {
        self.inet6_range = Some(range.into());
        self
    }
}

impl DefaultDnsRule {
    /// 创建新的默认 DNS 规则
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置域名匹配
    pub fn domain<S: Into<String>>(mut self, domain: S) -> Self {
        self.domain = Some(StringOrArray::Single(domain.into()));
        self
    }

    /// 设置多个域名匹配
    pub fn domains<I, S>(mut self, domains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.domain = Some(StringOrArray::Array(
            domains.into_iter().map(|s| s.into()).collect(),
        ));
        self
    }

    /// 设置域名后缀匹配
    pub fn domain_suffix<S: Into<String>>(mut self, suffix: S) -> Self {
        self.domain_suffix = Some(StringOrArray::Single(suffix.into()));
        self
    }

    /// 设置服务器
    pub fn server<S: Into<String>>(mut self, server: S) -> Self {
        self.server = Some(server.into());
        self
    }

    /// 设置规则动作
    pub fn action(mut self, action: DnsRuleAction) -> Self {
        self.action = Some(action);
        self
    }

    /// 反转匹配结果
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = Some(invert);
        self
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_config_serialize() {
        let dns = Dns {
            servers: Some(vec![]),
            rules: Some(vec![]),
            final_server: Some("local".to_string()),
            strategy: Some(DomainStrategy::PreferIpv4),
            disable_cache: Some(false),
            disable_expire: None,
            independent_cache: None,
            cache_capacity: Some(4096),
            reverse_mapping: Some(false),
            client_subnet: None,
            fakeip: None,
        };

        let json = serde_json::to_string_pretty(&dns).unwrap();
        assert!(json.contains("\"final\": \"local\""));
        assert!(json.contains("\"strategy\": \"prefer_ipv4\""));
    }

    #[test]
    fn test_dns_config_deserialize() {
        let json = r#"{
            "servers": [],
            "rules": [],
            "final": "local",
            "strategy": "prefer_ipv4",
            "disable_cache": false,
            "cache_capacity": 4096
        }"#;

        let dns: Dns = serde_json::from_str(json).unwrap();
        assert_eq!(dns.final_server, Some("local".to_string()));
        assert_eq!(dns.strategy, Some(DomainStrategy::PreferIpv4));
        assert_eq!(dns.cache_capacity, Some(4096));
    }

    #[test]
    fn test_local_dns_server() {
        let server = LocalDnsServer::new().tag("local").prefer_go(true);

        assert_eq!(server.tag, Some("local".to_string()));
        assert_eq!(server.prefer_go, Some(true));
    }

    #[test]
    fn test_hosts_dns_server_serialize() {
        let server = HostsDnsServer {
            tag: Some("hosts".to_string()),
            path: Some(StringOrArray::Array(vec![
                "/etc/hosts".to_string(),
                "$HOME/.hosts".to_string(),
            ])),
            predefined: None,
        };

        let json = serde_json::to_string(&server).unwrap();
        assert!(json.contains("\"tag\":\"hosts\""));
        assert!(json.contains("/etc/hosts"));
    }

    #[test]
    fn test_remote_dns_server() {
        let server = RemoteDnsServer::new()
            .tag("google")
            .server("8.8.8.8")
            .server_port(53)
            .strategy(DomainStrategy::Ipv4Only);

        assert_eq!(server.tag, Some("google".to_string()));
        assert_eq!(server.server, Some("8.8.8.8".to_string()));
        assert_eq!(server.server_port, Some(53));
    }

    #[test]
    fn test_fakeip_dns_server() {
        let server = FakeIPDnsServer::new()
            .tag("fakeip")
            .inet4_range("198.18.0.0/15")
            .inet6_range("fc00::/18");

        assert_eq!(server.tag, Some("fakeip".to_string()));
        assert_eq!(server.inet4_range, Some("198.18.0.0/15".to_string()));
        assert_eq!(server.inet6_range, Some("fc00::/18".to_string()));
    }

    #[test]
    fn test_legacy_fakeip() {
        let fakeip = LegacyFakeIP {
            enabled: Some(true),
            inet4_range: Some("198.18.0.0/15".to_string()),
            inet6_range: Some("fc00::/18".to_string()),
        };

        let json = serde_json::to_string(&fakeip).unwrap();
        assert!(json.contains("\"enabled\":true"));
    }

    #[test]
    fn test_default_dns_rule() {
        let rule = DefaultDnsRule::new().domain("google.com").server("google");

        assert_eq!(
            rule.domain,
            Some(StringOrArray::Single("google.com".to_string()))
        );
        assert_eq!(rule.server, Some("google".to_string()));
    }

    #[test]
    fn test_dns_rule_serialize() {
        let rule = DefaultDnsRule {
            domain: Some(StringOrArray::Array(vec![
                "google.com".to_string(),
                "youtube.com".to_string(),
            ])),
            server: Some("proxy".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("google.com"));
        assert!(json.contains("youtube.com"));
    }

    #[test]
    fn test_logical_dns_rule() {
        let rule = LogicalDnsRule {
            rule_type: "logical".to_string(),
            mode: LogicalMode::And,
            rules: vec![DefaultDnsRule {
                domain_suffix: Some(StringOrArray::Single(".cn".to_string())),
                ..Default::default()
            }],
            action: None,
            server: Some("local".to_string()),
        };

        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("\"mode\":\"and\""));
    }

    #[test]
    fn test_query_type() {
        //测试数字类型
        let qt: QueryTypeValue = serde_json::from_str("1").unwrap();
        assert_eq!(qt, QueryTypeValue::Number(1));

        // 测试字符串类型
        let qt: QueryTypeValue = serde_json::from_str("\"A\"").unwrap();
        assert_eq!(qt, QueryTypeValue::Name("A".to_string()));

        // 测试数组
        let qt: QueryType = serde_json::from_str("[\"A\", \"AAAA\",32768]").unwrap();
        if let QueryType::Array(arr) = qt {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_port_or_array() {
        // 单个端口
        let port: PortOrArray = serde_json::from_str("80").unwrap();
        assert_eq!(port, PortOrArray::Single(80));

        // 端口数组
        let ports: PortOrArray = serde_json::from_str("[80, 443]").unwrap();
        assert_eq!(ports, PortOrArray::Array(vec![80, 443]));
    }

    #[test]
    fn test_dns_builder() {
        let dns = Dns::new()
            .final_server("local")
            .strategy(DomainStrategy::PreferIpv4)
            .disable_cache(false)
            .cache_capacity(4096)
            .reverse_mapping(true);

        assert_eq!(dns.final_server, Some("local".to_string()));
        assert_eq!(dns.strategy, Some(DomainStrategy::PreferIpv4));
        assert_eq!(dns.disable_cache, Some(false));
        assert_eq!(dns.cache_capacity, Some(4096));
        assert_eq!(dns.reverse_mapping, Some(true));
    }

    #[test]
    fn test_dns_route_action() {
        let action = DnsRouteAction {
            server: Some("google".to_string()),
            disable_cache: Some(false),
            rewrite_ttl: Some(300),
            client_subnet: None,
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"server\":\"google\""));
        assert!(json.contains("\"rewrite_ttl\":300"));
    }

    #[test]
    fn test_dns_reject_action() {
        let action = DnsRejectAction {
            method: Some(DnsRejectMethod::Drop),
            no_drop: None,
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"method\":\"drop\""));
    }
}
