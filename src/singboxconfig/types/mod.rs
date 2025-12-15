//! sing-box 配置通用类型
//!
//! 此模块包含 sing-box 配置中使用的通用类型定义

mod domain_strategy;
mod duration;
mod network_strategy;
mod routing_mark;
mod string_or_array;
mod user;

pub use domain_strategy::DomainStrategy;
pub use duration::{Duration, ParseDurationError};
pub use network_strategy::{NetworkStrategy, NetworkType};
pub use routing_mark::RoutingMark;
pub use string_or_array::StringOrArray;
pub use user::{ShadowsocksDestination, UserWithPassword, VMessUser};
