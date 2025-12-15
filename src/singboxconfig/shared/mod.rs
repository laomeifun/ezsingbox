//! sing-box 共享配置类型
//!
//! 此模块包含入站和出站配置共享的类型定义

mod dial;
mod dns01_challenge;
mod listen;
mod multiplex;
mod tls;
mod v2ray;

pub use dial::{DialFields, DomainResolver, DomainResolverConfig};
pub use dns01_challenge::{AliDnsConfig, CloudflareConfig, Dns01Challenge};
pub use listen::ListenFields;
pub use multiplex::{MultiplexInbound, MultiplexOutbound, MultiplexProtocol, TcpBrutal};
pub use tls::{
    AcmeConfig, AcmeExternalAccount, AcmeProvider, AcmeProviderPreset, CipherSuite,
    ClientAuthentication, CurvePreference, EchInboundConfig, EchOutboundConfig, InboundTlsConfig,
    OutboundTlsConfig, RealityHandshake, RealityInboundConfig, RealityOutboundConfig, TlsVersion,
    UtlsConfig, UtlsFingerprint,
};
pub use v2ray::{
    GrpcTransport, HttpTransport, HttpUpgradeTransport, QuicTransport, V2RayTransport,
    WebSocketTransport,
};
