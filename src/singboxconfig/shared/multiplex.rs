use serde::{Deserialize, Serialize};

//============================================================================
// Multiplex 多路复用配置
// 文档: https://sing-box.sagernet.org/configuration/shared/multiplex/
//============================================================================

/// 入站多路复用配置
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexInbound {
    /// 启用多路复用支持
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 启用填充
    /// 如果启用，非填充连接将被拒绝
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<bool>,

    /// TCP Brutal 拥塞控制配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brutal: Option<TcpBrutal>,
}

/// 出站多路复用配置
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexOutbound {
    /// 启用多路复用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 多路复用协议
    /// 可选值: smux, yamux, h2mux
    /// 默认值: h2mux
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<MultiplexProtocol>,

    /// 最大连接数
    /// 与 max_streams 冲突
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,

    /// 打开新连接前，单个连接中的最小多路复用流数
    /// 与 max_streams 冲突
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_streams: Option<u32>,

    /// 打开新连接前，单个连接中的最大多路复用流数
    /// 与 max_connections 和 min_streams 冲突
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_streams: Option<u32>,

    /// 启用填充
    /// 需要 sing-box 服务端版本 1.3-beta9 或更高
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<bool>,

    /// TCP Brutal 拥塞控制配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brutal: Option<TcpBrutal>,
}

//============================================================================
// TCP Brutal 配置
// 文档: https://sing-box.sagernet.org/configuration/shared/tcp-brutal/
//============================================================================

/// TCP Brutal 拥塞控制配置
/// 服务器要求: Linux + brutal拥塞控制算法内核模块
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TcpBrutal {
    /// 启用 TCP Brutal 拥塞控制算法
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 上传带宽（Mbps）
    /// 启用时必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_mbps: Option<u32>,

    /// 下载带宽（Mbps）
    /// 启用时必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_mbps: Option<u32>,
}

//============================================================================
// 多路复用协议枚举
//============================================================================

/// 多路复用协议
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MultiplexProtocol {
    /// smux 协议
    /// https://github.com/xtaci/smux
    Smux,

    /// yamux 协议
    /// https://github.com/hashicorp/yamux
    Yamux,

    /// h2mux 协议（默认）
    /// https://golang.org/x/net/http2
    H2mux,
}

impl Default for MultiplexProtocol {
    fn default() -> Self {
        MultiplexProtocol::H2mux
    }
}

//============================================================================
// 构建器方法
//============================================================================

impl MultiplexInbound {
    /// 创建新的入站多路复用配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 启用多路复用
    pub fn enabled(mut self) -> Self {
        self.enabled = Some(true);
        self
    }

    /// 启用填充
    pub fn with_padding(mut self, padding: bool) -> Self {
        self.padding = Some(padding);
        self
    }

    /// 设置 TCP Brutal 配置
    pub fn with_brutal(mut self, brutal: TcpBrutal) -> Self {
        self.brutal = Some(brutal);
        self
    }
}

impl MultiplexOutbound {
    /// 创建新的出站多路复用配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 启用多路复用
    pub fn enabled(mut self) -> Self {
        self.enabled = Some(true);
        self
    }

    /// 设置协议
    pub fn with_protocol(mut self, protocol: MultiplexProtocol) -> Self {
        self.protocol = Some(protocol);
        self
    }

    /// 设置最大连接数
    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// 设置最小流数
    pub fn with_min_streams(mut self, min: u32) -> Self {
        self.min_streams = Some(min);
        self
    }

    /// 设置最大流数
    pub fn with_max_streams(mut self, max: u32) -> Self {
        self.max_streams = Some(max);
        self
    }

    /// 启用填充
    pub fn with_padding(mut self, padding: bool) -> Self {
        self.padding = Some(padding);
        self
    }

    /// 设置 TCP Brutal 配置
    pub fn with_brutal(mut self, brutal: TcpBrutal) -> Self {
        self.brutal = Some(brutal);
        self
    }

    /// 使用 smux 协议
    pub fn smux(mut self) -> Self {
        self.protocol = Some(MultiplexProtocol::Smux);
        self
    }

    /// 使用 yamux 协议
    pub fn yamux(mut self) -> Self {
        self.protocol = Some(MultiplexProtocol::Yamux);
        self
    }

    /// 使用 h2mux 协议
    pub fn h2mux(mut self) -> Self {
        self.protocol = Some(MultiplexProtocol::H2mux);
        self
    }
}

impl TcpBrutal {
    /// 创建新的 TCP Brutal 配置
    pub fn new(up_mbps: u32, down_mbps: u32) -> Self {
        Self {
            enabled: Some(true),
            up_mbps: Some(up_mbps),
            down_mbps: Some(down_mbps),
        }
    }

    /// 设置带宽
    pub fn with_bandwidth(mut self, up_mbps: u32, down_mbps: u32) -> Self {
        self.up_mbps = Some(up_mbps);
        self.down_mbps = Some(down_mbps);
        self
    }
}

//============================================================================
// 单元测试
//============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplex_inbound_serialize() {
        let mux = MultiplexInbound::new().enabled().with_padding(true);

        let json = serde_json::to_string(&mux).unwrap();
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"padding\":true"));
    }

    #[test]
    fn test_multiplex_inbound_deserialize() {
        let json = r#"{
            "enabled": true,
            "padding": false,
            "brutal": {
                "enabled": true,
                "up_mbps": 100,
                "down_mbps": 100
            }
        }"#;

        let mux: MultiplexInbound = serde_json::from_str(json).unwrap();
        assert_eq!(mux.enabled, Some(true));
        assert_eq!(mux.padding, Some(false));
        assert!(mux.brutal.is_some());
        let brutal = mux.brutal.unwrap();
        assert_eq!(brutal.up_mbps, Some(100));
        assert_eq!(brutal.down_mbps, Some(100));
    }

    #[test]
    fn test_multiplex_outbound_serialize() {
        let mux = MultiplexOutbound::new()
            .enabled()
            .smux()
            .with_max_connections(4)
            .with_min_streams(4);

        let json = serde_json::to_string(&mux).unwrap();
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"protocol\":\"smux\""));
        assert!(json.contains("\"max_connections\":4"));
        assert!(json.contains("\"min_streams\":4"));
    }

    #[test]
    fn test_multiplex_outbound_deserialize() {
        let json = r#"{
            "enabled": true,
            "protocol": "smux",
            "max_connections": 4,
            "min_streams": 4,
            "max_streams": 0,
            "padding": false
        }"#;

        let mux: MultiplexOutbound = serde_json::from_str(json).unwrap();
        assert_eq!(mux.enabled, Some(true));
        assert_eq!(mux.protocol, Some(MultiplexProtocol::Smux));
        assert_eq!(mux.max_connections, Some(4));
        assert_eq!(mux.min_streams, Some(4));
        assert_eq!(mux.max_streams, Some(0));
        assert_eq!(mux.padding, Some(false));
    }

    #[test]
    fn test_multiplex_protocol_serialize() {
        let smux = MultiplexProtocol::Smux;
        let yamux = MultiplexProtocol::Yamux;
        let h2mux = MultiplexProtocol::H2mux;

        assert_eq!(serde_json::to_string(&smux).unwrap(), "\"smux\"");
        assert_eq!(serde_json::to_string(&yamux).unwrap(), "\"yamux\"");
        assert_eq!(serde_json::to_string(&h2mux).unwrap(), "\"h2mux\"");
    }

    #[test]
    fn test_multiplex_protocol_deserialize() {
        let smux: MultiplexProtocol = serde_json::from_str("\"smux\"").unwrap();
        let yamux: MultiplexProtocol = serde_json::from_str("\"yamux\"").unwrap();
        let h2mux: MultiplexProtocol = serde_json::from_str("\"h2mux\"").unwrap();

        assert_eq!(smux, MultiplexProtocol::Smux);
        assert_eq!(yamux, MultiplexProtocol::Yamux);
        assert_eq!(h2mux, MultiplexProtocol::H2mux);
    }

    #[test]
    fn test_tcp_brutal_new() {
        let brutal = TcpBrutal::new(100, 200);

        assert_eq!(brutal.enabled, Some(true));
        assert_eq!(brutal.up_mbps, Some(100));
        assert_eq!(brutal.down_mbps, Some(200));
    }

    #[test]
    fn test_tcp_brutal_serialize() {
        let brutal = TcpBrutal::new(100, 100);

        let json = serde_json::to_string(&brutal).unwrap();
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"up_mbps\":100"));
        assert!(json.contains("\"down_mbps\":100"));
    }

    #[test]
    fn test_tcp_brutal_deserialize() {
        let json = r#"{
            "enabled": true,
            "up_mbps": 100,
            "down_mbps": 100
        }"#;

        let brutal: TcpBrutal = serde_json::from_str(json).unwrap();
        assert_eq!(brutal.enabled, Some(true));
        assert_eq!(brutal.up_mbps, Some(100));
        assert_eq!(brutal.down_mbps, Some(100));
    }

    #[test]
    fn test_multiplex_with_brutal() {
        let mux = MultiplexOutbound::new()
            .enabled()
            .h2mux()
            .with_brutal(TcpBrutal::new(100, 100));

        assert!(mux.brutal.is_some());
        let brutal = mux.brutal.unwrap();
        assert_eq!(brutal.up_mbps, Some(100));
    }

    #[test]
    fn test_default_protocol() {
        let protocol = MultiplexProtocol::default();
        assert_eq!(protocol, MultiplexProtocol::H2mux);
    }

    #[test]
    fn test_builder_chain() {
        let mux = MultiplexOutbound::new()
            .enabled()
            .yamux()
            .with_max_connections(8)
            .with_min_streams(2)
            .with_padding(true)
            .with_brutal(TcpBrutal::new(50, 50));

        assert_eq!(mux.enabled, Some(true));
        assert_eq!(mux.protocol, Some(MultiplexProtocol::Yamux));
        assert_eq!(mux.max_connections, Some(8));
        assert_eq!(mux.min_streams, Some(2));
        assert_eq!(mux.padding, Some(true));
        assert!(mux.brutal.is_some());
    }
}
