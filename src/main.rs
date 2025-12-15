mod autoconfig;
mod singboxconfig;

use autoconfig::{
    AutoDefault, DEFAULT_PORTS, MultiProtocolBuilder, quick_all, quick_anytls, quick_hysteria2,
    quick_tuic,
};

fn main() {
    println!("=== ezsingbox 自动化配置示例 ===\n");

    // 示例 1: 使用 quick_all() 一键部署所有协议
    println!("📦 示例 1: quick_all() - 一键部署所有协议");
    println!("{}", "-".repeat(50));

    match quick_all() {
        Ok(result) => {
            println!("✅ 配置生成成功！");
            println!("公网 IP: {}", result.public_ip);
            println!("   域名: {}", result.domain);

            if let Some(ref anytls) = result.anytls {
                println!("\n   🔐 AnyTLS:");
                println!("      端口: {}", anytls.info.port);
                println!("      用户数: {}", anytls.info.users.len());
                for user in &anytls.info.users {
                    println!("      - {} / {}", user.name, user.password);
                }
            }

            if let Some(ref hy2) = result.hysteria2 {
                println!("\n   🚀 Hysteria2:");
                println!("      端口: {}", hy2.info.port);
                println!("      用户数: {}", hy2.info.users.len());
                for user in &hy2.info.users {
                    println!("      - {} / {}", user.name, user.password);
                }
            }

            if let Some(ref tuic) = result.tuic {
                println!("\n   🌐 TUIC:");
                println!("      端口: {}", tuic.info.port);
                println!("      用户数: {}", tuic.info.users.len());
                for user in &tuic.info.users {
                    let uuid = user.uuid.as_deref().unwrap_or("N/A");
                    println!("      - {} / {} / {}", user.name, uuid, user.password);
                }
            }
        }
        Err(e) => {
            println!("❌ 配置生成失败: {}", e);
        }
    }

    // println!("\n");

    // // 示例 2: 单独部署各协议
    // println!("📦 示例 2: 单独部署各协议");
    // println!("{}", "-".repeat(50));

    // // AnyTLS
    // if let Ok(result) = quick_anytls() {
    //     println!("✅ AnyTLS: {}:{}", result.info.domain, result.info.port);
    // }

    // // Hysteria2
    // if let Ok(result) = quick_hysteria2() {
    //     println!("✅ Hysteria2: {}:{}", result.info.domain, result.info.port);
    // }

    // // TUIC
    // if let Ok(result) = quick_tuic() {
    //     println!("✅ TUIC: {}:{}", result.info.domain, result.info.port);
    // }

    // println!("\n");

    // // 示例 3: 自定义配置
    // println!("📦 示例 3: 自定义配置");
    // println!("{}", "-".repeat(50));

    // let ip: std::net::IpAddr = "1.2.3.4".parse().unwrap();

    // // 自定义 AnyTLS
    // if let Ok(result) = AutoDefault::anytls()
    //     .public_ip(ip)
    //     .port(443)
    //     .add_user("alice")
    //     .add_user_with_password("bob", "bob_password_123")
    //     .build_anytls()
    // {
    //     println!("✅ 自定义 AnyTLS:");
    //     println!("   域名: {}", result.info.domain);
    //     println!("   端口: {}", result.info.port);
    //     for user in &result.info.users {
    //         println!("   用户: {} / {}", user.name, user.password);
    //     }
    // }

    // // 自定义 Hysteria2(带混淆和带宽限制)
    // if let Ok(result) = AutoDefault::hysteria2()
    //     .public_ip(ip)
    //     .port(2053)
    //     .add_user("user1")
    //     .bandwidth(100, 100)
    //     .with_obfs()
    //     .masquerade("https://www.bing.com")
    //     .build_hysteria2()
    // {
    //     println!("\n✅ 自定义 Hysteria2:");
    //     println!("   域名: {}", result.info.domain);
    //     println!("   端口: {}", result.info.port);
    //     println!("   混淆密码: {:?}", result.obfs_password);
    //     println!(
    //         "   带宽: {}↑ / {}↓ Mbps",
    //         result.inbound.up_mbps.unwrap_or(0),
    //         result.inbound.down_mbps.unwrap_or(0)
    //     );
    // }

    // // 自定义 TUIC (使用 BBR)
    // if let Ok(result) = AutoDefault::tuic()
    //     .public_ip(ip)
    //     .port(2083)
    //     .add_user("tuic_user")
    //     .bbr()
    //     .build_tuic()
    // {
    //     println!("\n✅ 自定义 TUIC:");
    //     println!("   域名: {}", result.info.domain);
    //     println!("   端口: {}", result.info.port);
    //     println!("   拥塞控制: {:?}", result.inbound.congestion_control);
    //     for user in &result.info.users {
    //         let uuid = user.uuid.as_deref().unwrap_or("N/A");
    //         println!("   用户: {} / {} / {}", user.name, uuid, user.password);
    //     }
    // }

    // println!("\n");

    // // 示例 4: 多协议构建器
    // println!("📦 示例 4: MultiProtocolBuilder");
    // println!("{}", "-".repeat(50));

    // if let Ok(result) = MultiProtocolBuilder::new()
    //     .public_ip(ip)
    //     .enable_anytls(443)
    //     .enable_hysteria2(2053)
    //     .enable_tuic(2083)
    //     .add_user("shared_user")
    //     .hy2_bandwidth(200, 200)
    //     .tuic_congestion(singboxconfig::inbound::CongestionControl::Bbr)
    //     .build()
    // {
    //     println!("✅ 多协议配置生成成功！");
    //     println!("   公网 IP: {}", result.public_ip);
    //     println!("   域名: {}", result.domain);

    //     if result.anytls.is_some() {
    //         println!("✓ AnyTLS @443");
    //     }
    //     if result.hysteria2.is_some() {
    //         println!("   ✓ Hysteria2 @ 2053");
    //     }
    //     if result.tuic.is_some() {
    //         println!("   ✓ TUIC @ 2083");
    //     }
    // }

    // println!("\n");

    // // 示例 5: 输出 JSON 配置
    // println!("📦 示例 5: 输出 JSON 配置");
    // println!("{}", "-".repeat(50));

    // if let Ok(result) = AutoDefault::anytls()
    //     .public_ip(ip)
    //     .port(443)
    //     .add_user("demo_user")
    //     .build_anytls()
    // {
    //     match serde_json::to_string_pretty(&result.inbound) {
    //         Ok(json) => {
    //             println!("AnyTLS 入站配置 JSON:");
    //             println!("{}", json);
    //         }
    //         Err(e) => println!("JSON 序列化失败: {}", e),
    //     }
    // }

    // println!("\n");

    // // 显示默认端口列表
    // println!("📋 默认端口优先级: {:?}", DEFAULT_PORTS);
}
