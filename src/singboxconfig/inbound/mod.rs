//! sing-box 入站配置
//!
//! 此模块包含各种入站协议的配置定义

mod anytls;
mod hysteria2;

pub use anytls::AnyTlsInbound;
pub use hysteria2::{
    Hysteria2Inbound, Hysteria2Masquerade, Hysteria2MasqueradeConfig, Hysteria2Obfs, MasqueradeType,
};
