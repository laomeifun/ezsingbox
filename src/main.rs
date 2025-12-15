mod autoconfig;
mod dns;
mod singboxconfig;

use base64::Engine;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::path::Path;
use std::process::{Command, ExitCode};

use autoconfig::MultiProtocolBuilder;
use singboxconfig::full::SingBoxConfig;
use tiny_http::{Header, Method, Response, StatusCode};

fn env_bool(key: &str, default: bool) -> bool {
    match std::env::var(key) {
        Ok(raw) => {
            let v = raw.trim().to_ascii_lowercase();
            matches!(v.as_str(), "1" | "true" | "yes" | "y" | "on")
        }
        Err(_) => default,
    }
}

fn env_string(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn env_u16(key: &str) -> Option<u16> {
    env_string(key).and_then(|s| s.parse::<u16>().ok())
}

fn env_u32(key: &str) -> Option<u32> {
    env_string(key).and_then(|s| s.parse::<u32>().ok())
}

fn env_ip(key: &str) -> Option<IpAddr> {
    env_string(key).and_then(|s| s.parse::<IpAddr>().ok())
}

fn ensure_parent_dir(path: &str) -> std::io::Result<()> {
    let Some(parent) = Path::new(path).parent() else {
        return Ok(());
    };
    if parent.as_os_str().is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(parent)
}

fn pick_sing_box_bin() -> String {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClientProtocol {
    AnyTls,
    Hysteria2,
    Tuic,
    VlessReality,
}

impl ClientProtocol {
    fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "anytls" => Some(Self::AnyTls),
            "hysteria2" | "hy2" => Some(Self::Hysteria2),
            "tuic" => Some(Self::Tuic),
            "vless" | "vless-reality" | "vlessreality" | "reality" => Some(Self::VlessReality),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::AnyTls => "anytls",
            Self::Hysteria2 => "hysteria2",
            Self::Tuic => "tuic",
            Self::VlessReality => "vless-reality",
        }
    }
}

fn percent_encode(input: &str) -> String {
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

fn sing_box_import_remote_profile_uri(url: &str, name: &str) -> String {
    format!(
        "sing-box://import-remote-profile?url={}#{}",
        percent_encode(url),
        percent_encode(name)
    )
}

/// 生成 Hysteria2 分享链接
/// 格式: hysteria2://password@host:port?sni=xxx&insecure=0#name
fn generate_hysteria2_share_link(
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
fn generate_tuic_share_link(
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
fn generate_vless_reality_share_link(
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
fn generate_anytls_share_link(
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

fn build_from_env() -> Result<(autoconfig::MultiProtocolResult, String, bool, String), String> {
    let config_path = env_string("EZ_CONFIG_PATH").unwrap_or_else(|| "./config.json".to_string());
    let print_config = env_bool("EZ_PRINT_CONFIG", true);
    let log_level = env_string("EZ_LOG_LEVEL").unwrap_or_else(|| "info".to_string());

    let enable_anytls = env_bool("EZ_ENABLE_ANYTLS", true);
    let enable_hy2 = env_bool("EZ_ENABLE_HYSTERIA2", true);
    let enable_tuic = env_bool("EZ_ENABLE_TUIC", true);
    let enable_vless_reality = env_bool("EZ_ENABLE_VLESS_REALITY", true);

    let anytls_port = env_u16("EZ_ANYTLS_PORT").unwrap_or(443);
    let hy2_port = env_u16("EZ_HYSTERIA2_PORT").unwrap_or(2053);
    let tuic_port = env_u16("EZ_TUIC_PORT").unwrap_or(2083);
    let vless_reality_port = env_u16("EZ_VLESS_REALITY_PORT").unwrap_or(2096);

    let user = env_string("EZ_USER").unwrap_or_else(|| "default".to_string());
    let password = env_string("EZ_PASSWORD");

    let mut builder = MultiProtocolBuilder::new();
    if let Some(ip) = env_ip("EZ_PUBLIC_IP") {
        builder = builder.public_ip(ip);
    }
    if let Some(domain) = env_string("EZ_DOMAIN") {
        builder = builder.domain(domain);
    }

    if enable_anytls {
        builder = builder.enable_anytls(anytls_port);
    }
    if enable_hy2 {
        builder = builder.enable_hysteria2(hy2_port);
    }
    if enable_tuic {
        builder = builder.enable_tuic(tuic_port);
    }
    if enable_vless_reality {
        builder = builder.enable_vless_reality(vless_reality_port);
        //设置 VLESS Reality 握手服务器
        let handshake_server = env_string("EZ_VLESS_HANDSHAKE_SERVER")
            .unwrap_or_else(|| "www.microsoft.com".to_string());
        let handshake_port = env_u16("EZ_VLESS_HANDSHAKE_PORT").unwrap_or(443);
        builder = builder.vless_handshake(handshake_server, handshake_port);
    }

    if !enable_anytls && !enable_hy2 && !enable_tuic && !enable_vless_reality {
        builder = builder.enable_all();
    }

    builder = if let Some(pwd) = password {
        builder.add_user_with_password(user, pwd)
    } else {
        builder.add_user(user)
    };

    if env_bool("EZ_HY2_OBFS", false) {
        builder = builder.hy2_obfs();
    }
    if let (Some(up), Some(down)) = (env_u32("EZ_HY2_UP_MBPS"), env_u32("EZ_HY2_DOWN_MBPS")) {
        builder = builder.hy2_bandwidth(up, down);
    }

    if let Some(cc) = env_string("EZ_TUIC_CC") {
        builder = match cc.trim().to_ascii_lowercase().as_str() {
            "bbr" => builder.tuic_congestion(singboxconfig::inbound::CongestionControl::Bbr),
            "cubic" => builder.tuic_congestion(singboxconfig::inbound::CongestionControl::Cubic),
            "new_reno" | "newreno" => {
                builder.tuic_congestion(singboxconfig::inbound::CongestionControl::NewReno)
            }
            _ => builder,
        };
    }

    let result = builder.build().map_err(|e| e.to_string())?;
    Ok((result, config_path, print_config, log_level))
}

fn pick_client_protocol(result: &autoconfig::MultiProtocolResult) -> Option<ClientProtocol> {
    if let Some(raw) = env_string("EZ_CLIENT_PROTOCOL") {
        if let Some(p) = ClientProtocol::parse(&raw) {
            return Some(p);
        }
    }

    if result.anytls.is_some() {
        return Some(ClientProtocol::AnyTls);
    }
    if result.hysteria2.is_some() {
        return Some(ClientProtocol::Hysteria2);
    }
    if result.tuic.is_some() {
        return Some(ClientProtocol::Tuic);
    }
    if result.vless_reality.is_some() {
        return Some(ClientProtocol::VlessReality);
    }
    None
}

fn pick_user<'a>(users: &'a [autoconfig::GeneratedUser]) -> Option<&'a autoconfig::GeneratedUser> {
    if let Some(name) = env_string("EZ_CLIENT_USER") {
        if let Some(u) = users.iter().find(|u| u.name == name) {
            return Some(u);
        }
    }
    users.first()
}

fn build_proxy_outbound_json(
    result: &autoconfig::MultiProtocolResult,
    protocol: ClientProtocol,
    user: &autoconfig::GeneratedUser,
) -> Result<serde_json::Value, String> {
    let domain = &result.domain;
    let tls = serde_json::json!({
        "enabled": true,
        "server_name": domain
    });

    match protocol {
        ClientProtocol::AnyTls => {
            let anytls = result
                .anytls
                .as_ref()
                .ok_or_else(|| "AnyTLS 未启用".to_string())?;
            Ok(serde_json::json!({
                "type": "anytls",
                "tag": "proxy",
                "server": domain,
                "server_port": anytls.info.port,
                "password": user.password,
                "tls": tls
            }))
        }
        ClientProtocol::Hysteria2 => {
            let hy2 = result
                .hysteria2
                .as_ref()
                .ok_or_else(|| "Hysteria2 未启用".to_string())?;
            let mut v = serde_json::json!({
                "type": "hysteria2",
                "tag": "proxy",
                "server": domain,
                "server_port": hy2.info.port,
                "password": user.password,
                "tls": tls
            });

            if env_bool("EZ_HY2_OBFS", false) {
                if let Some(ref pwd) = hy2.obfs_password {
                    v["obfs"] = serde_json::json!({
                        "type": "salamander",
                        "password": pwd
                    });
                }
            }
            if let (Some(up), Some(down)) = (env_u32("EZ_HY2_UP_MBPS"), env_u32("EZ_HY2_DOWN_MBPS"))
            {
                v["up_mbps"] = serde_json::json!(up);
                v["down_mbps"] = serde_json::json!(down);
            }
            Ok(v)
        }
        ClientProtocol::Tuic => {
            let tuic = result
                .tuic
                .as_ref()
                .ok_or_else(|| "TUIC 未启用".to_string())?;
            let uuid = user
                .uuid
                .as_ref()
                .ok_or_else(|| "TUIC 用户缺少 UUID".to_string())?;

            let mut v = serde_json::json!({
                "type": "tuic",
                "tag": "proxy",
                "server": domain,
                "server_port": tuic.info.port,
                "uuid": uuid,
                "password": user.password,
                "tls": tls
            });

            if let Some(cc) = env_string("EZ_TUIC_CC") {
                let cc = cc.trim().to_ascii_lowercase();
                if matches!(cc.as_str(), "bbr" | "cubic" | "new_reno" | "newreno") {
                    let normalized = if cc == "newreno" {
                        "new_reno"
                    } else {
                        cc.as_str()
                    };
                    v["congestion_control"] = serde_json::json!(normalized);
                }
            }
            Ok(v)
        }
        ClientProtocol::VlessReality => {
            let vless = result
                .vless_reality
                .as_ref()
                .ok_or_else(|| "VLESS Reality 未启用".to_string())?;
            let uuid = user
                .uuid
                .as_ref()
                .ok_or_else(|| "VLESS用户缺少 UUID".to_string())?;

            // VLESS Reality 客户端配置
            Ok(serde_json::json!({
                "type": "vless",
                "tag": "proxy",
                "server": result.public_ip.to_string(),
                "server_port": vless.info.port,
                "uuid": uuid,
                "flow": "xtls-rprx-vision",
                "tls": {
                    "enabled": true,
                    "server_name": vless.handshake_server,
                    "utls": {
                        "enabled": true,
                        "fingerprint": "chrome"
                    },
                    "reality": {
                        "enabled": true,
                        "public_key": vless.public_key,
                        "short_id": vless.short_id
                    }
                }
            }))
        }
    }
}

fn generate_client_config_json(
    result: &autoconfig::MultiProtocolResult,
    log_level: &str,
) -> Result<(String, String), String> {
    let protocol =
        pick_client_protocol(result).ok_or_else(|| "没有可用协议用于生成客户端配置".to_string())?;

    let users: Vec<autoconfig::GeneratedUser> = match protocol {
        ClientProtocol::AnyTls => result
            .anytls
            .as_ref()
            .map(|r| r.info.users.clone())
            .unwrap_or_default(),
        ClientProtocol::Hysteria2 => result
            .hysteria2
            .as_ref()
            .map(|r| r.info.users.clone())
            .unwrap_or_default(),
        ClientProtocol::Tuic => result
            .tuic
            .as_ref()
            .map(|r| r.info.users.clone())
            .unwrap_or_default(),
        ClientProtocol::VlessReality => result
            .vless_reality
            .as_ref()
            .map(|r| r.info.users.clone())
            .unwrap_or_default(),
    };
    let user = pick_user(&users).ok_or_else(|| "没有可用用户用于生成客户端配置".to_string())?;

    let proxy = build_proxy_outbound_json(result, protocol, user)?;
    let mixed_listen =
        env_string("EZ_CLIENT_MIXED_LISTEN").unwrap_or_else(|| "127.0.0.1".to_string());
    let mixed_port = env_u16("EZ_CLIENT_MIXED_PORT").unwrap_or(7890);

    let cfg = SingBoxConfig::client_default(proxy, log_level, &mixed_listen, mixed_port);
    let json = cfg.to_pretty_json_string().map_err(|e| e.to_string())?;
    let profile_name = format!(
        "ezsingbox-{}-{}@{}",
        protocol.as_str(),
        user.name,
        result.domain
    );
    Ok((json, profile_name))
}

fn print_details(result: &autoconfig::MultiProtocolResult) {
    println!("\n==== 详细信息 (包含敏感信息) ====");
    println!("公网 IP: {}", result.public_ip);
    println!("域名: {}", result.domain);

    let domain = &result.domain;
    let public_ip_str = result.public_ip.to_string();

    // 获取 TUIC 拥塞控制算法
    let tuic_cc = env_string("EZ_TUIC_CC")
        .map(|s| s.trim().to_ascii_lowercase())
        .and_then(|cc| match cc.as_str() {
            "bbr" | "cubic" | "new_reno" | "newreno" => Some(if cc == "newreno" {
                "new_reno".to_string()
            } else {
                cc
            }),
            _ => None,
        });

    // 获取 Hysteria2 混淆密码
    let hy2_obfs_enabled = env_bool("EZ_HY2_OBFS", false);

    println!("\n==== 分享链接 ====");

    // AnyTLS 分享链接
    if let Some(ref anytls) = result.anytls {
        println!("\n[AnyTLS] 端口: {}", anytls.info.port);
        for u in &anytls.info.users {
            let link =
                generate_anytls_share_link(domain, anytls.info.port, &u.password, domain, &u.name);
            println!("用户 {}: {}", u.name, link);
        }
    }

    // Hysteria2 分享链接
    if let Some(ref hy2) = result.hysteria2 {
        println!("\n[Hysteria2] 端口: {}", hy2.info.port);
        let obfs_pwd = if hy2_obfs_enabled {
            hy2.obfs_password.as_deref()
        } else {
            None
        };
        for u in &hy2.info.users {
            let link = generate_hysteria2_share_link(
                domain,
                hy2.info.port,
                &u.password,
                domain,
                &u.name,
                obfs_pwd,
            );
            println!("  用户 {}: {}", u.name, link);
        }
    }

    // TUIC 分享链接
    if let Some(ref tuic) = result.tuic {
        println!("\n[TUIC] 端口: {}", tuic.info.port);
        for u in &tuic.info.users {
            if let Some(ref uuid) = u.uuid {
                let link = generate_tuic_share_link(
                    domain,
                    tuic.info.port,
                    uuid,
                    &u.password,
                    domain,
                    &u.name,
                    tuic_cc.as_deref(),
                );
                println!("  用户 {}: {}", u.name, link);
            }
        }
    }

    // VLESS Reality 分享链接
    if let Some(ref vless) = result.vless_reality {
        println!("\n[VLESS Reality] 端口: {}", vless.info.port);
        println!(
            "  握手服务器: {}:{}",
            vless.handshake_server, vless.handshake_port
        );
        println!("  公钥: {}", vless.public_key);
        println!("  短ID: {}", vless.short_id);
        for u in &vless.info.users {
            if let Some(ref uuid) = u.uuid {
                let link = generate_vless_reality_share_link(
                    &public_ip_str,
                    vless.info.port,
                    uuid,
                    &vless.public_key,
                    &vless.short_id,
                    &vless.handshake_server,
                    &u.name,
                );
                println!("  用户 {}: {}", u.name, link);
            }
        }
    }

    println!("\n==== 详细配置 ====");

    let mut print_users =
        |proto: ClientProtocol, port: u16, users: &[autoconfig::GeneratedUser]| {
            println!("\n[{}] 端口: {}", proto.as_str(), port);
            for u in users {
                println!("- 用户: {}", u.name);
                println!("  密码: {}", u.password);
                if let Some(ref uuid) = u.uuid {
                    println!("  UUID: {}", uuid);
                }
                if let Ok(outbound) = build_proxy_outbound_json(result, proto, u) {
                    if let Ok(s) = serde_json::to_string_pretty(&outbound) {
                        println!("  sing-box outbound:\n{}", s);
                    }
                }
            }
        };

    if let Some(ref anytls) = result.anytls {
        print_users(ClientProtocol::AnyTls, anytls.info.port, &anytls.info.users);
    }
    if let Some(ref hy2) = result.hysteria2 {
        print_users(ClientProtocol::Hysteria2, hy2.info.port, &hy2.info.users);
    }
    if let Some(ref tuic) = result.tuic {
        print_users(ClientProtocol::Tuic, tuic.info.port, &tuic.info.users);
    }
    if let Some(ref vless) = result.vless_reality {
        println!("\n[vless-reality] 端口: {}", vless.info.port);
        println!(
            "  握手服务器: {}:{}",
            vless.handshake_server, vless.handshake_port
        );
        println!("  公钥 (客户端使用): {}", vless.public_key);
        println!("  短ID: {}", vless.short_id);
        println!("  私钥 (服务端): {}", vless.private_key);
        for u in &vless.info.users {
            println!("- 用户: {}", u.name);
            if let Some(ref uuid) = u.uuid {
                println!("  UUID: {}", uuid);
            }
            if let Ok(outbound) = build_proxy_outbound_json(result, ClientProtocol::VlessReality, u)
            {
                if let Ok(s) = serde_json::to_string_pretty(&outbound) {
                    println!("  sing-box outbound:\n{}", s);
                }
            }
        }
    }

    if let Some(url) = env_string("EZ_REMOTE_PROFILE_URL") {
        let name = env_string("EZ_REMOTE_PROFILE_NAME").unwrap_or_else(|| "ezsingbox".to_string());
        println!("\n订阅链接: {}", url);
        println!(
            "URI 链接: {}",
            sing_box_import_remote_profile_uri(&url, &name)
        );
    }
}

fn generate_config_json(
    result: &autoconfig::MultiProtocolResult,
    log_level: &str,
) -> Result<String, String> {
    let mut inbounds = Vec::new();
    if let Some(ref anytls) = result.anytls {
        inbounds.push(serde_json::to_value(&anytls.inbound).map_err(|e| e.to_string())?);
    }
    if let Some(ref hy2) = result.hysteria2 {
        inbounds.push(serde_json::to_value(&hy2.inbound).map_err(|e| e.to_string())?);
    }
    if let Some(ref tuic) = result.tuic {
        inbounds.push(serde_json::to_value(&tuic.inbound).map_err(|e| e.to_string())?);
    }
    if let Some(ref vless) = result.vless_reality {
        inbounds.push(serde_json::to_value(&vless.inbound).map_err(|e| e.to_string())?);
    }

    let cfg = SingBoxConfig::server_default(inbounds, log_level);
    cfg.to_pretty_json_string().map_err(|e| e.to_string())
}

fn cmd_generate() -> Result<(), String> {
    let (result, config_path, print_config, log_level) = build_from_env()?;
    let json = generate_config_json(&result, &log_level)?;

    ensure_parent_dir(&config_path).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, &json).map_err(|e| e.to_string())?;

    println!("✅ sing-box 配置已生成: {}", config_path);
    println!("公网 IP: {}", result.public_ip);
    println!("域名: {}", result.domain);

    if let Some(ref anytls) = result.anytls {
        println!(
            "AnyTLS 端口: {} (tag={})",
            anytls.info.port, anytls.inbound.tag
        );
    }
    if let Some(ref hy2) = result.hysteria2 {
        println!(
            "Hysteria2 端口: {} (tag={})",
            hy2.info.port, hy2.inbound.tag
        );
    }
    if let Some(ref tuic) = result.tuic {
        println!("TUIC 端口: {} (tag={})", tuic.info.port, tuic.inbound.tag);
    }
    if let Some(ref vless) = result.vless_reality {
        println!(
            "VLESS-Reality 端口: {} (tag={})",
            vless.info.port, vless.inbound.tag
        );
    }

    if print_config {
        println!("\n{}", json);
    }

    if env_bool("EZ_PRINT_DETAILS", true) {
        print_details(&result);
    }

    if let Some(client_path) = env_string("EZ_CLIENT_CONFIG_PATH") {
        let (client_json, _name) = generate_client_config_json(&result, &log_level)?;
        ensure_parent_dir(&client_path).map_err(|e| e.to_string())?;
        std::fs::write(&client_path, &client_json).map_err(|e| e.to_string())?;
        println!("✅ client 配置已生成: {}", client_path);
    }

    Ok(())
}

fn cmd_run() -> Result<ExitCode, String> {
    let (result, config_path, print_config, log_level) = build_from_env()?;
    let json = generate_config_json(&result, &log_level)?;

    ensure_parent_dir(&config_path).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, &json).map_err(|e| e.to_string())?;

    if print_config {
        println!("\n{}", json);
    }

    if env_bool("EZ_PRINT_DETAILS", true) {
        print_details(&result);
    }

    let sing_box = pick_sing_box_bin();
    let status = Command::new(&sing_box)
        .arg("run")
        .arg("-c")
        .arg(&config_path)
        .status()
        .map_err(|e| format!("启动 sing-box 失败 ({}): {}", sing_box, e))?;

    let code: u8 = status
        .code()
        .and_then(|c| u8::try_from(c).ok())
        .unwrap_or(1);
    Ok(ExitCode::from(code))
}

fn cmd_serve() -> Result<ExitCode, String> {
    let (result, _config_path, _print_config, log_level) = build_from_env()?;

    let listen = env_string("EZ_SUBSCRIBE_LISTEN").unwrap_or_else(|| "0.0.0.0:8080".to_string());
    let listen_addr: SocketAddr = listen
        .parse()
        .map_err(|_| format!("EZ_SUBSCRIBE_LISTEN 无效: {}", listen))?;

    let path = env_string("EZ_SUBSCRIBE_PATH").unwrap_or_else(|| "/config.json".to_string());
    let path = if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    };

    let (client_json, profile_name) = generate_client_config_json(&result, &log_level)?;

    let public_url = env_string("EZ_SUBSCRIBE_PUBLIC_URL")
        .unwrap_or_else(|| format!("http://{}:{}{}", result.public_ip, listen_addr.port(), path));
    let import_name = env_string("EZ_SUBSCRIBE_NAME").unwrap_or(profile_name);

    println!("✅ 订阅服务已启动");
    println!("监听: {}", listen_addr);
    println!("路径: {}", path);
    println!("订阅链接: {}", public_url);
    println!(
        "URI 链接: {}",
        sing_box_import_remote_profile_uri(&public_url, &import_name)
    );

    let auth_user = env_string("EZ_SUBSCRIBE_BASIC_USER");
    let auth_pass = env_string("EZ_SUBSCRIBE_BASIC_PASS");
    let expected_auth = match (auth_user.as_deref(), auth_pass.as_deref()) {
        (Some(u), Some(p)) => {
            let token = base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", u, p));
            Some(format!("Basic {}", token))
        }
        _ => None,
    };

    let server = tiny_http::Server::http(listen_addr)
        .map_err(|e| format!("启动订阅 HTTP 服务失败: {}", e))?;
    for req in server.incoming_requests() {
        if req.method() != &Method::Get && req.method() != &Method::Head {
            let _ = req.respond(Response::empty(StatusCode(405)));
            continue;
        }
        if req.url() != path {
            let _ = req.respond(Response::empty(StatusCode(404)));
            continue;
        }

        if let Some(ref expected) = expected_auth {
            let provided = req
                .headers()
                .iter()
                .find(|h| h.field.equiv("Authorization"))
                .map(|h| h.value.as_str());
            if provided != Some(expected.as_str()) {
                let mut resp = Response::empty(StatusCode(401));
                let _ = resp.add_header(
                    Header::from_bytes(&b"WWW-Authenticate"[..], &b"Basic realm=\"ezsingbox\""[..])
                        .unwrap(),
                );
                let _ = req.respond(resp);
                continue;
            }
        }

        let mut resp = Response::from_string(client_json.clone());
        resp.add_header(
            Header::from_bytes(
                &b"Content-Type"[..],
                &b"application/json; charset=utf-8"[..],
            )
            .unwrap(),
        );
        let _ = req.respond(resp);
    }

    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    let mut args = std::env::args();
    let _exe = args.next();
    let sub = args.next().unwrap_or_else(|| "generate".to_string());

    match sub.as_str() {
        "generate" => match cmd_generate() {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("❌ {}", e);
                ExitCode::from(1)
            }
        },
        "run" => match cmd_run() {
            Ok(code) => code,
            Err(e) => {
                eprintln!("❌ {}", e);
                ExitCode::from(1)
            }
        },
        "serve" => match cmd_serve() {
            Ok(code) => code,
            Err(e) => {
                eprintln!("❌ {}", e);
                ExitCode::from(1)
            }
        },
        _ => {
            eprintln!(
                "用法: ezsingbox [generate|run|serve]\n\n环境变量(服务端生成): EZ_CONFIG_PATH, EZ_PUBLIC_IP, EZ_DOMAIN, EZ_ENABLE_ANYTLS, EZ_ENABLE_HYSTERIA2, EZ_ENABLE_TUIC, EZ_ANYTLS_PORT, EZ_HYSTERIA2_PORT, EZ_TUIC_PORT, EZ_USER, EZ_PASSWORD, EZ_HY2_OBFS, EZ_HY2_UP_MBPS, EZ_HY2_DOWN_MBPS, EZ_TUIC_CC, EZ_LOG_LEVEL, EZ_PRINT_CONFIG, EZ_PRINT_DETAILS\n\n环境变量(客户端导出): EZ_CLIENT_CONFIG_PATH, EZ_CLIENT_PROTOCOL, EZ_CLIENT_USER, EZ_CLIENT_MIXED_LISTEN, EZ_CLIENT_MIXED_PORT\n\n订阅/URI: EZ_REMOTE_PROFILE_URL, EZ_REMOTE_PROFILE_NAME\n\nHTTP订阅服务(serve): EZ_SUBSCRIBE_LISTEN, EZ_SUBSCRIBE_PATH, EZ_SUBSCRIBE_PUBLIC_URL, EZ_SUBSCRIBE_NAME, EZ_SUBSCRIBE_BASIC_USER, EZ_SUBSCRIBE_BASIC_PASS"
            );
            ExitCode::from(2)
        }
    }
}
