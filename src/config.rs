//! 配置构建和生成模块

use crate::autoconfig::{GeneratedUser, MultiProtocolBuilder, MultiProtocolResult};
use crate::env::{env_bool, env_ip, env_string, env_u16, env_u32};
use crate::protocol::ClientProtocol;
use crate::sharelink::{
    generate_anytls_share_link, generate_hysteria2_share_link, generate_tuic_share_link,
    generate_vless_reality_share_link, sing_box_import_remote_profile_uri,
};
use crate::singboxconfig::full::SingBoxConfig;
use crate::singboxconfig::inbound::CongestionControl;

/// 配置构建结果
pub struct BuildResult {
    pub result: MultiProtocolResult,
    pub config_path: String,
    pub print_config: bool,
    pub log_level: String,
}

/// 从环境变量构建配置
pub fn build_from_env() -> Result<BuildResult, String> {
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
    if let Some(email) = env_string("EZ_ACME_EMAIL") {
        builder = builder.acme_email(email);
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
        // 设置 VLESS Reality 握手服务器
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
            "bbr" => builder.tuic_congestion(CongestionControl::Bbr),
            "cubic" => builder.tuic_congestion(CongestionControl::Cubic),
            "new_reno" | "newreno" => builder.tuic_congestion(CongestionControl::NewReno),
            _ => builder,
        };
    }

    let result = builder.build().map_err(|e| e.to_string())?;
    Ok(BuildResult {
        result,
        config_path,
        print_config,
        log_level,
    })
}

/// 选择客户端协议
pub fn pick_client_protocol(result: &MultiProtocolResult) -> Option<ClientProtocol> {
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

/// 选择用户
pub fn pick_user(users: &[GeneratedUser]) -> Option<&GeneratedUser> {
    if let Some(name) = env_string("EZ_CLIENT_USER") {
        if let Some(u) = users.iter().find(|u| u.name == name) {
            return Some(u);
        }
    }
    users.first()
}

/// 构建代理出站 JSON
pub fn build_proxy_outbound_json(
    result: &MultiProtocolResult,
    protocol: ClientProtocol,
    user: &GeneratedUser,
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
                "tls": {
                    "enabled": true,
                    "server_name": domain,
                    "alpn": ["h3"]
                }
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

/// 生成客户端配置 JSON
pub fn generate_client_config_json(
    result: &MultiProtocolResult,
    log_level: &str,
) -> Result<(String, String), String> {
    let protocol =
        pick_client_protocol(result).ok_or_else(|| "没有可用协议用于生成客户端配置".to_string())?;

    let users: Vec<GeneratedUser> = match protocol {
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

/// 生成服务端配置 JSON
pub fn generate_config_json(
    result: &MultiProtocolResult,
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

/// 打印详细信息
pub fn print_details(result: &MultiProtocolResult) {
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

    let print_users = |proto: ClientProtocol, port: u16, users: &[GeneratedUser]| {
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
