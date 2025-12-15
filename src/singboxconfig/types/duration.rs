use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

//============================================================================
// Duration 时间间隔类型
// ============================================================================

/// sing-box 时间间隔类型
/// 支持格式: "1h", "30m", "5s", "300ms", "1h30m", "1m30s" 等
/// 文档: https://sing-box.sagernet.org/configuration/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Duration {
    /// 内部存储的毫秒数
    millis: u64,
    /// 原始字符串表示（用于保持序列化格式一致）
    raw: Option<String>,
}

impl Duration {
    /// 从毫秒创建
    pub fn from_millis(millis: u64) -> Self {
        Self { millis, raw: None }
    }

    /// 从秒创建
    pub fn from_secs(secs: u64) -> Self {
        Self {
            millis: secs * 1000,
            raw: None,
        }
    }

    /// 从分钟创建
    pub fn from_mins(mins: u64) -> Self {
        Self {
            millis: mins * 60 * 1000,
            raw: None,
        }
    }

    /// 从小时创建
    pub fn from_hours(hours: u64) -> Self {
        Self {
            millis: hours * 60 * 60 * 1000,
            raw: None,
        }
    }

    /// 获取毫秒数
    pub fn as_millis(&self) -> u64 {
        self.millis
    }

    /// 获取秒数
    pub fn as_secs(&self) -> u64 {
        self.millis / 1000
    }

    /// 获取分钟数
    pub fn as_mins(&self) -> u64 {
        self.millis / (60 * 1000)
    }

    /// 转换为标准库Duration
    pub fn to_std(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.millis)
    }

    /// 格式化为最简洁的字符串表示
    fn format_string(&self) -> String {
        if let Some(ref raw) = self.raw {
            return raw.clone();
        }

        let millis = self.millis;

        if millis == 0 {
            return "0s".to_string();
        }

        //尝试用最简洁的单位表示
        if millis % (60 * 60 * 1000) == 0 {
            format!("{}h", millis / (60 * 60 * 1000))
        } else if millis % (60 * 1000) == 0 {
            format!("{}m", millis / (60 * 1000))
        } else if millis % 1000 == 0 {
            format!("{}s", millis / 1000)
        } else {
            format!("{}ms", millis)
        }
    }
}

impl FromStr for Duration {
    type Err = ParseDurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ParseDurationError::Empty);
        }

        let mut total_millis: u64 = 0;
        let mut current_num = String::new();
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                current_num.push(c);
            } else if c.is_alphabetic() {
                if current_num.is_empty() {
                    return Err(ParseDurationError::InvalidFormat(s.to_string()));
                }

                let num: u64 = current_num
                    .parse()
                    .map_err(|_| ParseDurationError::InvalidNumber(current_num.clone()))?;
                current_num.clear();

                // 检查是否是"ms"
                let unit = if c == 'm' && chars.peek() == Some(&'s') {
                    chars.next(); // 消费 's'
                    "ms"
                } else {
                    match c {
                        'h' => "h",
                        'm' => "m",
                        's' => "s",
                        _ => return Err(ParseDurationError::InvalidUnit(c.to_string())),
                    }
                };

                let millis = match unit {
                    "h" => num * 60 * 60 * 1000,
                    "m" => num * 60 * 1000,
                    "s" => num * 1000,
                    "ms" => num,
                    _ => return Err(ParseDurationError::InvalidUnit(unit.to_string())),
                };

                total_millis = total_millis
                    .checked_add(millis)
                    .ok_or(ParseDurationError::Overflow)?;
            } else if !c.is_whitespace() {
                return Err(ParseDurationError::InvalidFormat(s.to_string()));
            }
        }

        // 如果还有未处理的数字，说明格式错误
        if !current_num.is_empty() {
            return Err(ParseDurationError::InvalidFormat(s.to_string()));
        }

        Ok(Duration {
            millis: total_millis,
            raw: Some(s.to_string()),
        })
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_string())
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.format_string())
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Duration::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl From<std::time::Duration> for Duration {
    fn from(d: std::time::Duration) -> Self {
        Duration::from_millis(d.as_millis() as u64)
    }
}

impl Default for Duration {
    fn default() -> Self {
        Duration::from_secs(0)
    }
}

// ============================================================================
// 解析错误类型
// ============================================================================

/// Duration 解析错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseDurationError {
    /// 空字符串
    Empty,
    /// 无效的数字
    InvalidNumber(String),
    /// 无效的单位
    InvalidUnit(String),
    /// 无效的格式
    InvalidFormat(String),
    /// 数值溢出
    Overflow,
}

impl fmt::Display for ParseDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseDurationError::Empty => write!(f, "空的时间间隔字符串"),
            ParseDurationError::InvalidNumber(n) => write!(f, "无效的数字: {}", n),
            ParseDurationError::InvalidUnit(u) => write!(f, "无效的时间单位: {}", u),
            ParseDurationError::InvalidFormat(s) => write!(f, "无效的时间格式: {}", s),
            ParseDurationError::Overflow => write!(f, "时间值溢出"),
        }
    }
}

impl std::error::Error for ParseDurationError {}

// ============================================================================
// 便捷宏
// ============================================================================

/// 创建 Duration 的便捷宏
///
/// # 示例
/// ```ignore
/// letd1 = duration!("5m");
/// let d2 = duration!("1h30m");
/// let d3 = duration!("300ms");
/// ```
#[macro_export]
macro_rules! duration {
    ($s:expr) => {
        $s.parse::<$crate::singboxconfig::types::Duration>()
            .unwrap()
    };
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        assert_eq!(Duration::from_str("5s").unwrap().as_millis(), 5000);
        assert_eq!(Duration::from_str("1m").unwrap().as_millis(), 60000);
        assert_eq!(Duration::from_str("2h").unwrap().as_millis(), 7200000);
        assert_eq!(Duration::from_str("300ms").unwrap().as_millis(), 300);
    }

    #[test]
    fn test_parse_compound() {
        assert_eq!(Duration::from_str("1h30m").unwrap().as_millis(), 5400000);
        assert_eq!(Duration::from_str("1m30s").unwrap().as_millis(), 90000);
        assert_eq!(Duration::from_str("1h30m45s").unwrap().as_millis(), 5445000);
    }

    #[test]
    fn test_format() {
        assert_eq!(Duration::from_secs(5).to_string(), "5s");
        assert_eq!(Duration::from_mins(1).to_string(), "1m");
        assert_eq!(Duration::from_hours(2).to_string(), "2h");
        assert_eq!(Duration::from_millis(300).to_string(), "300ms");
    }

    #[test]
    fn test_serialize() {
        let d = Duration::from_mins(5);
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(json, "\"5m\"");
    }

    #[test]
    fn test_deserialize() {
        let d: Duration = serde_json::from_str("\"5m\"").unwrap();
        assert_eq!(d.as_millis(), 300000);
    }

    #[test]
    fn test_preserves_raw() {
        let d = Duration::from_str("1h30m").unwrap();
        assert_eq!(d.to_string(), "1h30m");
    }
}
