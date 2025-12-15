use serde::{Deserialize, Serialize};

use crate::singboxconfig::types::{DomainStrategy, Duration, RoutingMark};

/// 入站监听字段配置
/// 文档: https://sing-box.sagernet.org/configuration/shared/listen/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListenFields {
    /// 监听地址（必填）
    pub listen: String,

    /// 监听端口
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<u16>,

    /// 要绑定的网络接口
    /// 自sing-box 1.12.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_interface: Option<String>,

    /// 设置 netfilter 路由标记（仅限 Linux）
    /// 支持整数（如 1234）和十六进制字符串（如 "0x1234"）
    /// 自 sing-box 1.12.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<RoutingMark>,

    /// 重用监听地址
    /// 自 sing-box 1.12.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_addr: Option<bool>,

    /// 设置网络命名空间，名称或路径（仅限 Linux）
    /// 自 sing-box 1.12.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netns: Option<String>,

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
    /// 默认值: 5m（在 1.13.0 中从 10m 更改为 5m）
    /// 自 sing-box 1.13.0 起可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_keep_alive: Option<Duration>,

    /// TCP 保活间隔
    /// 默认值: 75s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_keep_alive_interval: Option<Duration>,

    /// 启用 UDP 分片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_fragment: Option<bool>,

    /// UDP NAT 过期时间
    /// 默认值: 5m
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_timeout: Option<Duration>,

    ///如果设置，连接将被转发到指定的入站
    /// 需要目标入站支持（可注入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,

    // 已弃用字段（将在 sing-box 1.13.0 中移除）
    /// 启用协议嗅探
    /// 自 sing-box 1.11.0 起已弃用
    #[deprecated(since = "1.11.0", note = "请使用路由嗅探设置")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniff: Option<bool>,

    /// 使用嗅探到的域名覆盖连接目标地址
    /// 自 sing-box 1.11.0 起已弃用
    #[deprecated(since = "1.11.0", note = "请使用路由嗅探设置")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniff_override_destination: Option<bool>,

    /// 嗅探超时时间
    /// 默认值: 300ms
    /// 自 sing-box 1.11.0 起已弃用
    #[deprecated(since = "1.11.0", note = "请使用路由嗅探设置")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniff_timeout: Option<Duration>,

    /// 域名解析策略
    /// 自 sing-box 1.11.0 起已弃用
    #[deprecated(since = "1.11.0", note = "请使用路由嗅探设置")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_strategy: Option<DomainStrategy>,

    /// 如果启用，对于地址为域名的 UDP 代理请求，
    /// 将在响应中发送原始数据包地址而不是映射的域名
    /// 此选项用于兼容不支持接收域名地址 UDP 数据包的客户端，如 Surge
    /// 自 sing-box 1.11.0 起已弃用
    #[deprecated(since = "1.11.0", note = "请使用路由嗅探设置")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_disable_domain_unmapping: Option<bool>,
}

impl Default for ListenFields {
    fn default() -> Self {
        Self {
            listen: "::".to_string(),
            listen_port: None,
            bind_interface: None,
            routing_mark: None,
            reuse_addr: None,
            netns: None,
            tcp_fast_open: None,
            tcp_multi_path: None,
            disable_tcp_keep_alive: None,
            tcp_keep_alive: None,
            tcp_keep_alive_interval: None,
            udp_fragment: None,
            udp_timeout: None,
            detour: None,
            sniff: None,
            sniff_override_destination: None,
            sniff_timeout: None,
            domain_strategy: None,
            udp_disable_domain_unmapping: None,
        }
    }
}
