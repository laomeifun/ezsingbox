//! 命令处理模块

use std::net::SocketAddr;
use std::process::{Command, ExitCode};

use base64::Engine;
use tiny_http::{Header, Method, Response, StatusCode};

use crate::config::{
    build_from_env, generate_client_config_json, generate_config_json, print_details,
};
use crate::env::{env_bool, env_string};
use crate::sharelink::sing_box_import_remote_profile_uri;
use crate::utils::{ensure_parent_dir, pick_sing_box_bin};

/// 生成配置命令
pub fn cmd_generate() -> Result<(), String> {
    let build_result = build_from_env()?;
    let result = &build_result.result;
    let config_path = &build_result.config_path;
    let print_config = build_result.print_config;
    let log_level = &build_result.log_level;

    let json = generate_config_json(result, log_level)?;

    ensure_parent_dir(config_path).map_err(|e| e.to_string())?;
    std::fs::write(config_path, &json).map_err(|e| e.to_string())?;

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
        print_details(result);
    }

    if let Some(client_path) = env_string("EZ_CLIENT_CONFIG_PATH") {
        let (client_json, _name) = generate_client_config_json(result, log_level)?;
        ensure_parent_dir(&client_path).map_err(|e| e.to_string())?;
        std::fs::write(&client_path, &client_json).map_err(|e| e.to_string())?;
        println!("✅ client配置已生成: {}", client_path);
    }

    Ok(())
}

/// 运行 sing-box 命令
pub fn cmd_run() -> Result<ExitCode, String> {
    let build_result = build_from_env()?;
    let result = &build_result.result;
    let config_path = &build_result.config_path;
    let print_config = build_result.print_config;
    let log_level = &build_result.log_level;

    let json = generate_config_json(result, log_level)?;

    ensure_parent_dir(config_path).map_err(|e| e.to_string())?;
    std::fs::write(config_path, &json).map_err(|e| e.to_string())?;

    if print_config {
        println!("\n{}", json);
    }

    if env_bool("EZ_PRINT_DETAILS", true) {
        print_details(result);
    }

    let sing_box = pick_sing_box_bin();
    let status = Command::new(&sing_box)
        .arg("run")
        .arg("-c")
        .arg(config_path)
        .status()
        .map_err(|e| format!("启动 sing-box 失败({}): {}", sing_box, e))?;

    let code: u8 = status
        .code()
        .and_then(|c| u8::try_from(c).ok())
        .unwrap_or(1);
    Ok(ExitCode::from(code))
}

/// 订阅服务命令
pub fn cmd_serve() -> Result<ExitCode, String> {
    let build_result = build_from_env()?;
    let result = &build_result.result;
    let log_level = &build_result.log_level;

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

    let (client_json, profile_name) = generate_client_config_json(result, log_level)?;

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

/// 打印帮助信息
pub fn print_usage() {
    eprintln!(
        "用法: ezsingbox [generate|run|serve]\n\n\
        环境变量(服务端生成): EZ_CONFIG_PATH, EZ_PUBLIC_IP, EZ_DOMAIN, EZ_ENABLE_ANYTLS, \
        EZ_ENABLE_HYSTERIA2, EZ_ENABLE_TUIC, EZ_ANYTLS_PORT, EZ_HYSTERIA2_PORT, EZ_TUIC_PORT, \
        EZ_USER, EZ_PASSWORD, EZ_HY2_OBFS, EZ_HY2_UP_MBPS, EZ_HY2_DOWN_MBPS, EZ_TUIC_CC, \
        EZ_LOG_LEVEL, EZ_PRINT_CONFIG, EZ_PRINT_DETAILS\n\n\
        环境变量(客户端导出): EZ_CLIENT_CONFIG_PATH, EZ_CLIENT_PROTOCOL, EZ_CLIENT_USER, \
        EZ_CLIENT_MIXED_LISTEN, EZ_CLIENT_MIXED_PORT\n\n\
        订阅/URI: EZ_REMOTE_PROFILE_URL, EZ_REMOTE_PROFILE_NAME\n\n\
        HTTP订阅服务(serve): EZ_SUBSCRIBE_LISTEN, EZ_SUBSCRIBE_PATH, EZ_SUBSCRIBE_PUBLIC_URL, \
        EZ_SUBSCRIBE_NAME, EZ_SUBSCRIBE_BASIC_USER, EZ_SUBSCRIBE_BASIC_PASS"
    );
}
