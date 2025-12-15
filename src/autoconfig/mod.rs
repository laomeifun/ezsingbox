//! 自动化配置生成模块
//!
//! 此模块提供自动化生成 sing-box 配置的功能

mod autoanytls;
mod autohy2;
pub mod tools;

// 从 autoanytls 模块导出
pub use autoanytls::{
    AutoAnyTlsBuilder, AutoAnyTlsConfig, AutoAnyTlsError, AutoAnyTlsResult, ConnectionInfo,
};

// 从 autohy2 模块导出
pub use autohy2::{
    AutoHysteria2Builder, AutoHysteria2Config, AutoHysteria2Error, AutoHysteria2Result,
    Hysteria2ConnectionInfo,
};

// 从 tools 模块重新导出常用功能
pub use tools::{
    PublicIpError, TlsMode, UserConfig, generate_hex_string, generate_nip_domain,
    generate_password, generate_password_with_length, generate_random_bytes, generate_sslip_domain,
    generate_uuid, generate_uuid_simple, get_public_ip, get_public_ip_with_timeout,
};
