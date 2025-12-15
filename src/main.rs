mod autoconfig;
mod singboxconfig;

use autoconfig::{AutoAnyTlsBuilder, get_public_ip};

fn main() {
    println!("===== AnyTLS 自动配置生成器 =====\n");

    // 尝试自动获取公网 IP
    println!("正在获取公网 IP...");
    match get_public_ip() {
        Ok(ip) => {
            println!("检测到公网 IP: {}\n", ip);

            // 使用自动检测的 IP生成配置
            let result = AutoAnyTlsBuilder::new()
                .public_ip(ip)
                .add_user("default_user")
                .build()
                .unwrap();

            // 生成的配置
            println!("===== 生成的入站配置 =====\n");
            println!(
                "{}\n",
                serde_json::to_string_pretty(&result.inbound).unwrap()
            );

            // 查看生成的用户密码
            println!("===== 用户信息 =====\n");
            for user in &result.users {
                println!("用户名: {}", user.name);
                println!("密码: {}\n", user.password);
            }

            // 查看连接信息
            println!("===== 连接信息 =====\n");
            println!("服务器: {}", result.connection_info.server);
            println!("端口: {}", result.connection_info.port);
            if let Some(sni) = &result.connection_info.server_name {
                println!("SNI: {}", sni);
            }
        }
        Err(e) => {
            println!("无法自动获取公网 IP: {}", e);
            println!("请手动指定 IP 地址");

            // 使用示例 IP 演示
            println!("\n===== 使用示例 IP 演示 =====\n");
            let result = AutoAnyTlsBuilder::new()
                .public_ip("203.0.113.1".parse().unwrap())
                .add_user("demo_user")
                .build()
                .unwrap();

            println!("{}", serde_json::to_string_pretty(&result.inbound).unwrap());
        }
    }
}
