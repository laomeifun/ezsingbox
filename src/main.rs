mod autoconfig;
mod singboxconfig;

use autoconfig::{AutoAnyTlsBuilder, AutoHysteria2Builder, get_public_ip};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║sing-box 自动化配置生成器演示                ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // 尝试自动获取公网 IP
    println!("🌐 正在获取公网 IP...");
    match get_public_ip() {
        Ok(ip) => {
            println!("✅ 检测到公网 IP: {}\n", ip);

            // 演示 AnyTLS 配置生成
            demo_anytls(ip);

            println!("\n{}\n", "─".repeat(60));

            // 演示 Hysteria2 配置生成
            demo_hysteria2(ip);
        }
        Err(e) => {
            println!("❌ 无法自动获取公网 IP: {}", e);
            println!("📝 使用示例 IP进行演示...\n");

            let demo_ip: std::net::IpAddr = "203.0.113.1".parse().unwrap();

            demo_anytls(demo_ip);
            println!("\n{}\n", "─".repeat(60));
            demo_hysteria2(demo_ip);
        }
    }
}

/// 演示 AnyTLS 配置生成
fn demo_anytls(ip: std::net::IpAddr) {
    println!("╭──────────────────────────────────────────────────────────────╮");
    println!("│AnyTLS 配置生成演示                       │");
    println!("╰──────────────────────────────────────────────────────────────╯\n");

    let result = AutoAnyTlsBuilder::new()
        .public_ip(ip)
        .port(443)
        .add_user("user1")
        .add_user_with_password("user2", "my_custom_password")
        .build();

    match result {
        Ok(result) => {
            // 打印生成的配置
            println!("📄 生成的入站配置:");
            println!(
                "{}\n",
                serde_json::to_string_pretty(&result.inbound).unwrap()
            );

            // 打印用户信息
            println!("👥 用户信息:");
            println!("┌─────────────┬──────────────────────────────┐");
            println!("│ 用户名      │ 密码                         │");
            println!("├─────────────┼──────────────────────────────┤");
            for user in &result.users {
                println!("│ {:11} │ {:28} │", user.name, user.password);
            }
            println!("└─────────────┴──────────────────────────────┘\n");

            // 打印连接信息
            println!("🔗 连接信息:");
            println!("   服务器: {}", result.connection_info.server);
            println!("   端口: {}", result.connection_info.port);
            if let Some(sni) = &result.connection_info.server_name {
                println!("   SNI: {}", sni);
            }
        }
        Err(e) => {
            println!("❌ 生成配置失败: {}", e);
        }
    }
}

/// 演示 Hysteria2 配置生成
fn demo_hysteria2(ip: std::net::IpAddr) {
    println!("╭──────────────────────────────────────────────────────────────╮");
    println!("│Hysteria2 配置生成演示                      │");
    println!("╰──────────────────────────────────────────────────────────────╯\n");

    let result = AutoHysteria2Builder::new()
        .public_ip(ip)
        .port(443)
        .bandwidth(100, 100) // 上下行带宽限制 100Mbps
        .with_obfs_password("my_obfs_secret") // 启用混淆
        .with_masquerade("https://www.bing.com") // 伪装网站
        .add_user("hy2_user1")
        .add_user_with_password("hy2_user2", "custom_hy2_password")
        .build();

    match result {
        Ok(result) => {
            // 打印生成的配置
            println!("📄 生成的入站配置:");
            println!(
                "{}\n",
                serde_json::to_string_pretty(&result.inbound).unwrap()
            );

            // 打印用户信息
            println!("👥 用户信息:");
            println!("┌─────────────┬──────────────────────────────┐");
            println!("│ 用户名      │ 密码                         │");
            println!("├─────────────┼──────────────────────────────┤");
            for user in &result.users {
                println!("│ {:11} │ {:28} │", user.name, user.password);
            }
            println!("└─────────────┴──────────────────────────────┘\n");

            // 打印连接信息
            println!("🔗 连接信息:");
            println!("   服务器: {}", result.connection_info.server);
            println!("   端口: {}", result.connection_info.port);
            if let Some(sni) = &result.connection_info.server_name {
                println!("   SNI: {}", sni);
            }
            if let Some(up) = result.connection_info.up_mbps {
                println!("   上行带宽: {} Mbps", up);
            }
            if let Some(down) = result.connection_info.down_mbps {
                println!("   下行带宽: {} Mbps", down);
            }
            println!(
                "   混淆: {}",
                if result.connection_info.obfs_enabled {
                    "已启用"
                } else {
                    "未启用"
                }
            );
            if let Some(obfs_pwd) = &result.obfs_password {
                println!("   混淆密码: {}", obfs_pwd);
            }
        }
        Err(e) => {
            println!("❌ 生成配置失败: {}", e);
        }
    }
}
