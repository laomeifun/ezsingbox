use serde::{Deserialize, Serialize};

//============================================================================
// 字符串或字符串数组类型
// ============================================================================

/// 字符串或字符串数组
/// 用于处理可以是单个字符串或字符串数组的字段
///
/// 在 sing-box 配置中，某些字段（如证书、密钥等）可以接受单个字符串或字符串数组
///
/// # 示例
///
/// 单个字符串:
/// ```json
/// "certificate": "-----BEGIN CERTIFICATE-----\n..."
/// ```
///
/// 字符串数组:
/// ```json
/// "certificate": [
///     "-----BEGIN CERTIFICATE-----",
///     "...",
///     "-----END CERTIFICATE-----"
/// ]
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringOrArray {
    /// 单个字符串
    Single(String),
    /// 字符串数组
    Array(Vec<String>),
}

impl StringOrArray {
    /// 创建单个字符串
    pub fn single<S: Into<String>>(s: S) -> Self {
        StringOrArray::Single(s.into())
    }

    /// 创建字符串数组
    pub fn array<I, S>(iter: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        StringOrArray::Array(iter.into_iter().map(|s| s.into()).collect())
    }

    /// 判断是否为单个字符串
    pub fn is_single(&self) -> bool {
        matches!(self, StringOrArray::Single(_))
    }

    /// 判断是否为数组
    pub fn is_array(&self) -> bool {
        matches!(self, StringOrArray::Array(_))
    }

    /// 获取为单个字符串（如果是数组则返回None）
    pub fn as_single(&self) -> Option<&str> {
        match self {
            StringOrArray::Single(s) => Some(s),
            StringOrArray::Array(_) => None,
        }
    }

    /// 获取为数组（如果是单个字符串则返回 None）
    pub fn as_array(&self) -> Option<&[String]> {
        match self {
            StringOrArray::Single(_) => None,
            StringOrArray::Array(arr) => Some(arr),
        }
    }

    /// 转换为字符串向量
    pub fn into_vec(self) -> Vec<String> {
        match self {
            StringOrArray::Single(s) => vec![s],
            StringOrArray::Array(arr) => arr,
        }
    }

    /// 获取所有字符串的引用（无论是单个还是数组）
    pub fn as_slice(&self) -> Vec<&str> {
        match self {
            StringOrArray::Single(s) => vec![s.as_str()],
            StringOrArray::Array(arr) => arr.iter().map(|s| s.as_str()).collect(),
        }
    }

    /// 合并为单个字符串（使用换行符连接）
    pub fn join(&self, separator: &str) -> String {
        match self {
            StringOrArray::Single(s) => s.clone(),
            StringOrArray::Array(arr) => arr.join(separator),
        }
    }
}

impl From<String> for StringOrArray {
    fn from(s: String) -> Self {
        StringOrArray::Single(s)
    }
}

impl From<&str> for StringOrArray {
    fn from(s: &str) -> Self {
        StringOrArray::Single(s.to_string())
    }
}

impl From<Vec<String>> for StringOrArray {
    fn from(v: Vec<String>) -> Self {
        StringOrArray::Array(v)
    }
}

impl From<Vec<&str>> for StringOrArray {
    fn from(v: Vec<&str>) -> Self {
        StringOrArray::Array(v.into_iter().map(|s| s.to_string()).collect())
    }
}

impl Default for StringOrArray {
    fn default() -> Self {
        StringOrArray::Single(String::new())
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        let s = StringOrArray::single("hello");
        assert!(s.is_single());
        assert!(!s.is_array());
        assert_eq!(s.as_single(), Some("hello"));assert_eq!(s.as_array(), None);
    }

    #[test]
    fn test_array() {
        let s = StringOrArray::array(vec!["a", "b", "c"]);
        assert!(!s.is_single());
        assert!(s.is_array());
        assert_eq!(s.as_single(), None);
        assert_eq!(
            s.as_array(),
            Some(vec!["a".to_string(), "b".to_string(), "c".to_string()].as_slice())
        );
    }

    #[test]
    fn test_into_vec() {
        let single = StringOrArray::single("hello");
        assert_eq!(single.into_vec(), vec!["hello".to_string()]);

        let array = StringOrArray::array(vec!["a", "b"]);
        assert_eq!(array.into_vec(), vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_join() {
        let single = StringOrArray::single("hello");
        assert_eq!(single.join("\n"), "hello");

        let array = StringOrArray::array(vec!["a", "b", "c"]);
        assert_eq!(array.join("\n"), "a\nb\nc");
    }

    #[test]
    fn test_serialize_single() {
        let s = StringOrArray::single("hello");
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "\"hello\"");
    }

    #[test]
    fn test_serialize_array() {
        let s = StringOrArray::array(vec!["a", "b"]);
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "[\"a\",\"b\"]");
    }

    #[test]
    fn test_deserialize_single() {
        let s: StringOrArray = serde_json::from_str("\"hello\"").unwrap();
        assert_eq!(s.as_single(), Some("hello"));
    }

    #[test]
    fn test_deserialize_array() {
        let s: StringOrArray = serde_json::from_str("[\"a\",\"b\"]").unwrap();
        assert_eq!(
            s.as_array(),
            Some(vec!["a".to_string(), "b".to_string()].as_slice())
        );
    }
}
