//! 自动化配置生成模块
//!
//! 此模块提供自动化生成 sing-box 配置的功能

#[allow(non_snake_case)]
mod autoByDefault;
mod autoanytls;
mod autohy2;
mod autotuic;
mod autovless;
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

// 从 autotuic 模块导出
pub use autotuic::{
    AutoTuicBuilder, AutoTuicConfig, AutoTuicError, AutoTuicResult, TuicConnectionInfo,
    TuicUserConfig,
};

// 从 autovless 模块导出
pub use autovless::{
    AutoVlessBuilder, AutoVlessConfig, AutoVlessError, AutoVlessResult, VlessConnectionInfo,
    VlessUserConfig,
};

// 从 autoByDefault 模块导出
pub use autoByDefault::{
    // 结果类型
    AnyTlsAutoResult,
    AutoBuildResult,
    // 构建器
    AutoDefault,
    // 辅助类型
    AutoDefaultError,
    AutoDefaultResult,
    // 端口常量和函数
    DEFAULT_PORTS,
    GeneratedUser,
    Hysteria2AutoResult,
    MultiProtocolBuilder,
    MultiProtocolResult,
    Protocol,
    TuicAutoResult,
    default_port,
    fallback_port,
    // 便捷函数
    quick_all,
    quick_anytls,
    quick_hysteria2,
    quick_tuic,
};

// 从 tools 模块重新导出常用功能
pub use tools::{
    PublicIpError, TlsMode, UserConfig, generate_hex_string, generate_nip_domain,
    generate_password, generate_password_with_length, generate_random_bytes, generate_sslip_domain,
    generate_uuid, generate_uuid_simple, get_public_ip, get_public_ip_with_timeout,
};
