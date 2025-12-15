//! 客户端协议枚举模块

/// 客户端协议类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientProtocol {
    AnyTls,
    Hysteria2,
    Tuic,
    VlessReality,
}

impl ClientProtocol {
    /// 从字符串解析协议类型
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "anytls" => Some(Self::AnyTls),
            "hysteria2" | "hy2" => Some(Self::Hysteria2),
            "tuic" => Some(Self::Tuic),
            "vless" | "vless-reality" | "vlessreality" | "reality" => Some(Self::VlessReality),
            _ => None,
        }
    }

    /// 获取协议名称字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AnyTls => "anytls",
            Self::Hysteria2 => "hysteria2",
            Self::Tuic => "tuic",
            Self::VlessReality => "vless-reality",
        }
    }
}
