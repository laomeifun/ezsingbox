//! 文件系统和工具函数模块

use std::path::Path;

use crate::env::env_string;

/// 确保父目录存在
pub fn ensure_parent_dir(path: &str) -> std::io::Result<()> {
    let Some(parent) = Path::new(path).parent() else {
        return Ok(());
    };
    if parent.as_os_str().is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(parent)
}

/// 选择 sing-box 二进制文件路径
pub fn pick_sing_box_bin() -> String {
    if let Some(v) = env_string("SING_BOX_BIN") {
        return v;
    }

    for cand in [
        "sing-box",
        "/usr/bin/sing-box",
        "/bin/sing-box",
        "/sing-box",
    ] {
        if cand.starts_with('/') {
            if Path::new(cand).exists() {
                return cand.to_string();
            }
        } else {
            return cand.to_string();
        }
    }
    "sing-box".to_string()
}
