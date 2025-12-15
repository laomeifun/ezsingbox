use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::singboxconfig::types::Duration;

//============================================================================
// V2Ray Transport 配置
// 文档: https://sing-box.sagernet.org/configuration/shared/v2ray-transport/
//============================================================================

/// V2Ray Transport 配置
/// 支持的传输类型: HTTP, WebSocket, QUIC, gRPC, HTTPUpgrade
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum V2RayTransport {
    /// HTTP 传输
    Http(HttpTransport),
    /// WebSocket 传输
    Ws(WebSocketTransport),
    /// QUIC 传输
    Quic(QuicTransport),
    /// gRPC 传输
    Grpc(GrpcTransport),
    /// HTTPUpgrade 传输
    #[serde(rename = "httpupgrade")]
    HttpUpgrade(HttpUpgradeTransport),
}

//============================================================================
// HTTP 传输
//============================================================================

/// HTTP 传输配置
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpTransport {
    /// 主机域名列表
    /// 客户端将随机选择，服务端将验证（如果非空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<String>>,

    /// HTTP 请求路径
    /// 服务端将验证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// HTTP 请求方法
    /// 服务端将验证（如果非空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// HTTP 请求额外头部
    /// 服务端将在响应中写入（如果非空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// 空闲超时时间
    /// HTTP2 服务端: 指定空闲客户端应该被GOAWAY 帧关闭的时间
    /// HTTP2 客户端: 指定在连接上没有收到帧后执行健康检查的时间间隔
    /// 默认值: 0（不执行健康检查）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<Duration>,

    /// Ping 超时时间
    /// HTTP2 客户端: 发送 PING 帧后必须收到响应的超时时间
    /// 默认值: 15s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_timeout: Option<Duration>,
}

//============================================================================
// WebSocket 传输
//============================================================================

/// WebSocket 传输配置
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebSocketTransport {
    /// HTTP 请求路径
    /// 服务端将验证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// HTTP 请求额外头部
    /// 服务端将在响应中写入（如果非空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// 允许在请求中发送的有效载荷大小
    /// 非零时启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_early_data: Option<u32>,

    /// 早期数据头部名称
    /// 默认情况下早期数据在路径中发送而不是头部
    /// 要与 Xray-core 兼容，设置为 "Sec-WebSocket-Protocol"
    /// 需要与服务端保持一致
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_data_header_name: Option<String>,
}

//============================================================================
// QUIC 传输
//============================================================================

/// QUIC 传输配置
/// 注意: 不支持额外加密（基本上是重复加密）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuicTransport {
    // QUIC 传输没有额外配置字段
}

//============================================================================
// gRPC 传输
//============================================================================

/// gRPC 传输配置
/// 注意: 标准 gRPC 有良好的兼容性但性能较差，默认不包含
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrpcTransport {
    /// gRPC 服务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,

    /// 空闲超时时间
    /// 标准 gRPC: 如果传输在此时间内没有看到任何活动，它会ping 客户端以检查连接是否仍然活跃
    /// 默认gRPC: 与 HTTP 传输中的相应设置行为相同
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<Duration>,

    /// Ping 超时时间
    /// 标准 gRPC: 执行 keepalive 检查后，客户端等待活动的超时时间
    /// 默认 gRPC: 与 HTTP 传输中的相应设置行为相同
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_timeout: Option<Duration>,

    /// 允许无流时发送 keepalive ping
    /// 标准 gRPC 客户端: 如果启用，即使没有活动连接也会发送 keepalive ping
    /// 如果禁用，当没有活动连接时，idle_timeout 和 ping_timeout 将被忽略
    /// 默认值: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permit_without_stream: Option<bool>,
}

//============================================================================
// HTTPUpgrade 传输
//============================================================================

/// HTTPUpgrade 传输配置
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpUpgradeTransport {
    /// 主机域名
    /// 服务端将验证（如果非空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// HTTP 请求路径
    /// 服务端将验证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// HTTP 请求额外头部
    /// 服务端将在响应中写入（如果非空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

//============================================================================
// 构建器方法
//============================================================================

impl V2RayTransport {
    /// 创建 HTTP 传输
    pub fn http() -> Self {
        V2RayTransport::Http(HttpTransport::default())
    }

    /// 创建 WebSocket 传输
    pub fn ws() -> Self {
        V2RayTransport::Ws(WebSocketTransport::default())
    }

    /// 创建 QUIC 传输
    pub fn quic() -> Self {
        V2RayTransport::Quic(QuicTransport::default())
    }

    /// 创建 gRPC 传输
    pub fn grpc() -> Self {
        V2RayTransport::Grpc(GrpcTransport::default())
    }

    /// 创建 HTTPUpgrade 传输
    pub fn http_upgrade() -> Self {
        V2RayTransport::HttpUpgrade(HttpUpgradeTransport::default())
    }
}

impl HttpTransport {
    /// 创建新的 HTTP 传输配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置主机列表
    pub fn with_host(mut self, host: Vec<String>) -> Self {
        self.host = Some(host);
        self
    }

    /// 设置路径
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// 设置方法
    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    /// 设置头部
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// 添加单个头部
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// 设置空闲超时
    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = Some(timeout);
        self
    }

    /// 设置 ping 超时
    pub fn with_ping_timeout(mut self, timeout: Duration) -> Self {
        self.ping_timeout = Some(timeout);
        self
    }
}

impl WebSocketTransport {
    /// 创建新的 WebSocket 传输配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置路径
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// 设置头部
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// 添加单个头部
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// 设置最大早期数据大小
    pub fn with_max_early_data(mut self, size: u32) -> Self {
        self.max_early_data = Some(size);
        self
    }

    /// 设置早期数据头部名称
    pub fn with_early_data_header_name(mut self, name: impl Into<String>) -> Self {
        self.early_data_header_name = Some(name.into());
        self
    }

    /// 设置为 Xray 兼容模式
    pub fn xray_compatible(mut self) -> Self {
        self.early_data_header_name = Some("Sec-WebSocket-Protocol".to_string());
        self
    }
}

impl GrpcTransport {
    /// 创建新的 gRPC 传输配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置服务名称
    pub fn with_service_name(mut self, name: impl Into<String>) -> Self {
        self.service_name = Some(name.into());
        self
    }

    /// 设置空闲超时
    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = Some(timeout);
        self
    }

    /// 设置 ping 超时
    pub fn with_ping_timeout(mut self, timeout: Duration) -> Self {
        self.ping_timeout = Some(timeout);
        self
    }

    /// 设置允许无流时发送 keepalive
    pub fn with_permit_without_stream(mut self, permit: bool) -> Self {
        self.permit_without_stream = Some(permit);
        self
    }
}

impl HttpUpgradeTransport {
    /// 创建新的 HTTPUpgrade 传输配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置主机
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    /// 设置路径
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// 设置头部
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// 添加单个头部
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
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
    fn test_http_transport_serialize() {
        let transport = V2RayTransport::Http(
            HttpTransport::new()
                .with_host(vec!["example.com".to_string()])
                .with_path("/v2ray"),
        );

        let json = serde_json::to_string(&transport).unwrap();
        assert!(json.contains("\"type\":\"http\""));
        assert!(json.contains("\"path\":\"/v2ray\""));
    }

    #[test]
    fn test_http_transport_deserialize() {
        let json = r#"{
            "type": "http",
            "host": ["example.com"],
            "path": "/v2ray",
            "method": "GET"
        }"#;

        let transport: V2RayTransport = serde_json::from_str(json).unwrap();
        if let V2RayTransport::Http(http) = transport {
            assert_eq!(http.path, Some("/v2ray".to_string()));
            assert_eq!(http.method, Some("GET".to_string()));
        } else {
            panic!("Expected HTTP transport");
        }
    }

    #[test]
    fn test_ws_transport_serialize() {
        let transport = V2RayTransport::Ws(
            WebSocketTransport::new()
                .with_path("/ws")
                .with_max_early_data(2048)
                .xray_compatible(),
        );

        let json = serde_json::to_string(&transport).unwrap();
        assert!(json.contains("\"type\":\"ws\""));
        assert!(json.contains("\"path\":\"/ws\""));
        assert!(json.contains("\"max_early_data\":2048"));
        assert!(json.contains("\"early_data_header_name\":\"Sec-WebSocket-Protocol\""));
    }

    #[test]
    fn test_ws_transport_deserialize() {
        let json = r#"{
            "type": "ws",
            "path": "/ws",
            "max_early_data": 2048,
            "early_data_header_name": "Sec-WebSocket-Protocol"
        }"#;

        let transport: V2RayTransport = serde_json::from_str(json).unwrap();
        if let V2RayTransport::Ws(ws) = transport {
            assert_eq!(ws.path, Some("/ws".to_string()));
            assert_eq!(ws.max_early_data, Some(2048));
            assert_eq!(
                ws.early_data_header_name,
                Some("Sec-WebSocket-Protocol".to_string())
            );
        } else {
            panic!("Expected WebSocket transport");
        }
    }

    #[test]
    fn test_quic_transport_serialize() {
        let transport = V2RayTransport::quic();

        let json = serde_json::to_string(&transport).unwrap();
        assert!(json.contains("\"type\":\"quic\""));
    }

    #[test]
    fn test_quic_transport_deserialize() {
        let json = r#"{"type": "quic"}"#;

        let transport: V2RayTransport = serde_json::from_str(json).unwrap();
        assert!(matches!(transport, V2RayTransport::Quic(_)));
    }

    #[test]
    fn test_grpc_transport_serialize() {
        let transport = V2RayTransport::Grpc(
            GrpcTransport::new()
                .with_service_name("TunService")
                .with_permit_without_stream(true),
        );

        let json = serde_json::to_string(&transport).unwrap();
        assert!(json.contains("\"type\":\"grpc\""));
        assert!(json.contains("\"service_name\":\"TunService\""));
        assert!(json.contains("\"permit_without_stream\":true"));
    }

    #[test]
    fn test_grpc_transport_deserialize() {
        let json = r#"{
            "type": "grpc",
            "service_name": "TunService",
            "idle_timeout": "15s",
            "ping_timeout": "15s",
            "permit_without_stream": false
        }"#;

        let transport: V2RayTransport = serde_json::from_str(json).unwrap();
        if let V2RayTransport::Grpc(grpc) = transport {
            assert_eq!(grpc.service_name, Some("TunService".to_string()));
            assert_eq!(grpc.permit_without_stream, Some(false));
        } else {
            panic!("Expected gRPC transport");
        }
    }

    #[test]
    fn test_httpupgrade_transport_serialize() {
        let transport = V2RayTransport::HttpUpgrade(
            HttpUpgradeTransport::new()
                .with_host("example.com")
                .with_path("/upgrade"),
        );

        let json = serde_json::to_string(&transport).unwrap();
        assert!(json.contains("\"type\":\"httpupgrade\""));
        assert!(json.contains("\"host\":\"example.com\""));
        assert!(json.contains("\"path\":\"/upgrade\""));
    }

    #[test]
    fn test_httpupgrade_transport_deserialize() {
        let json = r#"{
            "type": "httpupgrade",
            "host": "example.com",
            "path": "/upgrade"
        }"#;

        let transport: V2RayTransport = serde_json::from_str(json).unwrap();
        if let V2RayTransport::HttpUpgrade(upgrade) = transport {
            assert_eq!(upgrade.host, Some("example.com".to_string()));
            assert_eq!(upgrade.path, Some("/upgrade".to_string()));
        } else {
            panic!("Expected HTTPUpgrade transport");
        }
    }

    #[test]
    fn test_http_transport_with_headers() {
        let transport = HttpTransport::new()
            .with_path("/v2ray")
            .add_header("X-Custom-Header", "custom-value")
            .add_header("User-Agent", "Mozilla/5.0");

        assert!(transport.headers.is_some());
        let headers = transport.headers.unwrap();
        assert_eq!(
            headers.get("X-Custom-Header"),
            Some(&"custom-value".to_string())
        );
        assert_eq!(headers.get("User-Agent"), Some(&"Mozilla/5.0".to_string()));
    }

    #[test]
    fn test_factory_methods() {
        let http = V2RayTransport::http();
        assert!(matches!(http, V2RayTransport::Http(_)));

        let ws = V2RayTransport::ws();
        assert!(matches!(ws, V2RayTransport::Ws(_)));

        let quic = V2RayTransport::quic();
        assert!(matches!(quic, V2RayTransport::Quic(_)));

        let grpc = V2RayTransport::grpc();
        assert!(matches!(grpc, V2RayTransport::Grpc(_)));

        let upgrade = V2RayTransport::http_upgrade();
        assert!(matches!(upgrade, V2RayTransport::HttpUpgrade(_)));
    }
}
