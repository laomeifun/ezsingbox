use serde::{Deserialize, Serialize};

//============================================================================
// 路由标记类型
// ============================================================================

/// 路由标记类型
/// 用于设置 netfilter 路由标记（仅限 Linux）
/// 支持整数（如 1234）和十六进制字符串（如 "0x1234"）
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RoutingMark {
    /// 整数形式（如 1234）
    Int(u32),
    /// 十六进制字符串形式（如 "0x1234"）
    Hex(String),
}

impl RoutingMark {
    /// 从整数创建
    pub fn from_int(value: u32) -> Self {
        RoutingMark::Int(value)
    }

    /// 从十六进制字符串创建
    pub fn from_hex(value: impl Into<String>) -> Self {
        RoutingMark::Hex(value.into())
    }

    /// 获取数值（如果是十六进制字符串则解析）
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            RoutingMark::Int(v) => Some(*v),
            RoutingMark::Hex(s) => {
                let s = s.trim_start_matches("0x").trim_start_matches("0X");
                u32::from_str_radix(s, 16).ok()
            }
        }
    }
}

impl From<u32> for RoutingMark {
    fn from(value: u32) -> Self {
        RoutingMark::Int(value)
    }
}

impl From<&str> for RoutingMark {
    fn from(value: &str) -> Self {
        RoutingMark::Hex(value.to_string())
    }
}

impl From<String> for RoutingMark {
    fn from(value: String) -> Self {
        RoutingMark::Hex(value)
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_int() {
        let mark = RoutingMark::from_int(1234);
        assert_eq!(mark.as_u32(), Some(1234));
    }

    #[test]
    fn test_from_hex() {
        let mark = RoutingMark::from_hex("0x1234");
        assert_eq!(mark.as_u32(), Some(0x1234));
    }

    #[test]
    fn test_serialize_int() {
        let mark = RoutingMark::Int(1234);
        let json = serde_json::to_string(&mark).unwrap();
        assert_eq!(json, "1234");
    }

    #[test]
    fn test_serialize_hex() {
        let mark = RoutingMark::Hex("0x1234".to_string());
        let json = serde_json::to_string(&mark).unwrap();
        assert_eq!(json, "\"0x1234\"");
    }

    #[test]
    fn test_deserialize_int() {
        let mark: RoutingMark = serde_json::from_str("1234").unwrap();
        assert!(matches!(mark, RoutingMark::Int(1234)));
    }

    #[test]
    fn test_deserialize_hex() {
        let mark: RoutingMark = serde_json::from_str("\"0x1234\"").unwrap();
        assert!(matches!(mark, RoutingMark::Hex(_)));
    }
}
