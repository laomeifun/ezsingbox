//! 分享链接生成模块

/// URL 百分号编码
pub fn percent_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for b in input.as_bytes() {
        let c = *b as char;
        let is_unreserved = matches!(c,
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~'
        );
        if is_unreserved {
            out.push(c);
        } else {
            out.push('%');
            out.push_str(&format!("{:02X}", b));
        }
    }
    out
}

/// 生成 sing-box 导入远程配置的 URI
pub fn sing_box_import_remote_profile_uri(url: &str, name: &str) -> String {
    format!(
        "sing-box://import-remote-profile?url={}#{}",
        percent_encode(url),
        percent_encode(name)
    )
}

/// 生成 Hysteria2 分享链接
/// 格式: hysteria2://password@host:port?sni=xxx&insecure=0#name
pub fn generate_hysteria2_share_link(
    host: &str,
    port: u16,
    password: &str,
    sni: &str,
    name: &str,
    obfs_password: Option<&str>,
) -> String {
    let mut params = format!("sni={}&insecure=0", percent_encode(sni));
    if let Some(obfs_pwd) = obfs_password {
        params.push_str(&format!(
            "&obfs=salamander&obfs-password={}",
            percent_encode(obfs_pwd)
        ));
    }
    format!(
        "hysteria2://{}@{}:{}?{}#{}",
        percent_encode(password),
        host,
        port,
        params,
        percent_encode(name)
    )
}

/// 生成 TUIC 分享链接
/// 格式: tuic://uuid:password@host:port?sni=xxx&congestion_control=bbr&udp_relay_mode=native&alpn=h3#name
pub fn generate_tuic_share_link(
    host: &str,
    port: u16,
    uuid: &str,
    password: &str,
    sni: &str,
    name: &str,
    congestion_control: Option<&str>,
) -> String {
    let cc = congestion_control.unwrap_or("bbr");
    format!(
        "tuic://{}:{}@{}:{}?sni={}&congestion_control={}&udp_relay_mode=native&alpn=h3#{}",
        percent_encode(uuid),
        percent_encode(password),
        host,
        port,
        percent_encode(sni),
        cc,
        percent_encode(name)
    )
}

/// 生成 VLESS Reality 分享链接
/// 格式: vless://uuid@host:port?encryption=none&type=tcp&security=reality&pbk=xxx&sid=xxx&sni=xxx&fp=chrome&flow=xtls-rprx-vision#name
pub fn generate_vless_reality_share_link(
    host: &str,
    port: u16,
    uuid: &str,
    public_key: &str,
    short_id: &str,
    sni: &str,
    name: &str,
) -> String {
    format!(
        "vless://{}@{}:{}?encryption=none&type=tcp&security=reality&pbk={}&sid={}&sni={}&fp=chrome&flow=xtls-rprx-vision#{}",
        uuid,
        host,
        port,
        percent_encode(public_key),
        short_id,
        percent_encode(sni),
        percent_encode(name)
    )
}

/// 生成 AnyTLS 分享链接
/// 格式: anytls://password@host:port?sni=xxx&insecure=0#name
pub fn generate_anytls_share_link(
    host: &str,
    port: u16,
    password: &str,
    sni: &str,
    name: &str,
) -> String {
    format!(
        "anytls://{}@{}:{}?sni={}&insecure=0#{}",
        percent_encode(password),
        host,
        port,
        percent_encode(sni),
        percent_encode(name)
    )
}
