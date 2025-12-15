//! sing-box 共享配置类型
//!
//! 此模块包含入站和出站配置共享的类型定义

mod dial;
mod listen;
mod tls;

pub use dial::{DialFields, DomainResolver, DomainResolverConfig};
pub use listen::ListenFields;
pub use tls::{
    AcmeConfig, AcmeExternalAccount, AcmeProvider, AcmeProviderPreset, CipherSuite,
    ClientAuthentication, CurvePreference, EchInboundConfig, EchOutboundConfig, InboundTlsConfig,
    OutboundTlsConfig, RealityHandshake, RealityInboundConfig, RealityOutboundConfig, TlsVersion,
    UtlsConfig, UtlsFingerprint,
};
