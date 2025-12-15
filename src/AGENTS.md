# ezsingbox - sing-box 自动化配置生成器

## 项目概述

**ezsingbox** 是一个用 Rust 编写的 sing-box 配置自动化生成工具。它提供了简单易用的 API 来生成 sing-box 的入站配置,特别是针对 **AnyTLS** 和 **Hysteria2** 协议。

### 核心功能

- 🚀 **自动化配置生成**: 通过 Builder 模式快速生成 sing-box 入站配置
- 🔐 **用户管理**: 自动生成用户密码,支持自定义密码
- 🌐 **公网 IP 自动检测**: 自动从多个服务获取公网 IP
- 🔧 **TLS 自动化**: 支持 ACME 自动证书申请和自签名证书
- 📦 **域名生成**: 自动生成 sslip.io/nip.io 域名用于 TLS
- 🎯 **协议支持**: AnyTLS 和 Hysteria2 协议

### 技术栈

- **语言**: Rust (Edition 2024)
- **核心依赖**:
  - `serde` / `serde_json`: 配置序列化
  - `rustls-acme`: ACME 证书自动化
  - `ureq`: HTTP 客户端(用于获取公网 IP)
  - `x25519-dalek`: 密钥生成
  - `uuid`: UUID 生成
  - `base64`: Base64 编解码
  - `rand`: 随机数生成

## 项目结构

```
src/
├── main.rs                    # 主程序入口,包含演示代码
├── mod.rs                     # 模块声明
├── autoconfig/                # 自动化配置生成模块
│   ├── mod.rs                 # 模块导出
│   ├── autoanytls.rs          # AnyTLS 配置生成器
│   ├── autohy2.rs             # Hysteria2 配置生成器
│   └── tools.rs               # 通用工具函数
└── singboxconfig/             # sing-box 配置数据模型
    ├── lib.rs                 # 库入口
    ├── mod.rs                 # 模块声明
    ├── inbound/               # 入站配置
    │   ├── mod.rs
    │   ├── anytls.rs          # AnyTLS 入站配置
    │   └── hysteria2.rs       # Hysteria2 入站配置
    ├── outbound/              # 出站配置
    │   ├── mod.rs
    │   └── anytls.rs          # AnyTLS 出站配置
    ├── shared/                # 共享配置
    │   ├── mod.rs
    │   ├── tls.rs             # TLS 配置
    │   ├── dns01_challenge.rs # DNS-01 挑战配置
    │   ├── listen.rs          # 监听配置
    │   ├── dial.rs            # 拨号配置
    │   ├── multiplex.rs       # 多路复用配置
    │   └── v2ray.rs           # V2Ray 传输配置
    └── types/                 # 自定义类型
        ├── mod.rs
        ├── user.rs            # 用户类型
        ├── duration.rs        # 时长类型
        ├── domain_strategy.rs # 域名策略
        ├── network_strategy.rs# 网络策略
        ├── routing_mark.rs    # 路由标记
        └── string_or_array.rs # 字符串或数组类型
```

## 构建和运行

### 开发环境

- Rust 工具链 (推荐使用 rustup)
- Edition: 2024

### 常用命令

```bash
# 检查代码
cargo check --all-features

# 运行测试
cargo test --all-features

# 构建 Debug 版本
cargo build

# 构建 Release 版本
cargo build --release

# 运行演示程序
cargo run

# 运行 Release 版本
cargo run --release
```

### 代码检查和格式化

```bash
# 格式化代码
cargo fmt

# Lint 检查
cargo clippy

# 完整检查(推荐在提交前运行)
cargo fmt && cargo clippy && cargo test
```

## 使用示例

### AnyTLS 配置生成

```rust
use ezsingbox::autoconfig::AutoAnyTlsBuilder;

let result = AutoAnyTlsBuilder::new()
    .public_ip("203.0.113.1".parse().unwrap())
    .port(443)
    .add_user("user1")
    .add_user_with_password("user2", "custom_password")
    .build()?;

// 获取生成的入站配置
let inbound_config = result.inbound;
// 获取用户信息
let users = result.users;
// 获取连接信息
let connection_info = result.connection_info;
```

### Hysteria2 配置生成

```rust
use ezsingbox::autoconfig::AutoHysteria2Builder;

let result = AutoHysteria2Builder::new()
    .public_ip("203.0.113.1".parse().unwrap())
    .port(443)
    .bandwidth(100, 100)  // 上下行 100Mbps
    .with_obfs_password("obfs_secret")  // 启用混淆
    .with_masquerade("https://www.bing.com")  // 伪装网站
    .add_user("hy2_user1")
    .build()?;
```

### 自动获取公网 IP

```rust
use ezsingbox::autoconfig::get_public_ip;

let ip = get_public_ip()?;
println!("公网 IP: {}", ip);
```

## 开发约定

### 代码风格

- 遵循 Rust 标准命名约定
- 使用 `cargo fmt` 格式化代码
- 通过 `cargo clippy` 检查
- 为公共 API 编写文档注释 (`///`)
- 为模块编写模块级文档 (`//!`)

### 模块组织

- `autoconfig`: 高级自动化配置生成 API
- `singboxconfig`: 底层 sing-box 配置数据结构
- 使用 Builder 模式提供流畅的 API
- 错误处理使用 `Result<T, E>` 模式

### 测试

- 单元测试放在对应模块的 `#[cfg(test)]` 块中
- 集成测试放在 `tests/` 目录(如果有)
- 运行测试: `cargo test`

## CI/CD

### GitHub Actions 工作流

项目配置了两个主要的 CI/CD 工作流:

#### 1. CI 工作流 (`.github/workflows/ci.yml`)

触发条件: 推送到 `main`/`master`/`develop` 分支或 PR

- **Check**: 代码检查 (`cargo check --all-features`)
- **Test**: 运行测试 (`cargo test --all-features`)
- **Build**: 构建 Release 版本 (`cargo build --release`)

#### 2. Release 工作流 (`.github/workflows/release.yml`)

触发条件: 推送 `v*` 标签(如 `v0.1.0`)或手动触发

跨平台构建支持:
- **Linux**: x86_64, aarch64
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Windows**: x86_64

自动创建 GitHub Release 并上传编译好的二进制文件。

### 发布新版本

```bash
# 1. 更新版本号 (Cargo.toml)
# 2. 提交更改
git add .
git commit -m "chore: bump version to 0.1.0"

# 3. 创建并推送标签
git tag v0.1.0
git push origin v0.1.0

# GitHub Actions 会自动构建并发布
```

## 项目特点

### Builder 模式

项目广泛使用 Builder 模式提供流畅的 API:

```rust
AutoAnyTlsBuilder::new()
    .port(443)
    .public_ip(ip)
    .add_user("user1")
    .build()?
```

### 自动化功能

- **自动密码生成**: 如果不提供密码,自动生成强密码
- **自动域名生成**: 基于 IP 生成 sslip.io 域名
- **自动 IP 检测**: 从多个服务获取公网 IP,提高可靠性
- **自动 TLS 配置**: 支持 ACME 自动证书和自签名证书

### 错误处理

使用自定义错误类型提供清晰的错误信息:

```rust
pub enum AutoAnyTlsError {
    MissingPublicIp,
    AcmeDomainRequired,
    InvalidConfiguration(String),
}
```

## 相关资源

- **sing-box 官方文档**: https://sing-box.sagernet.org/
- **AnyTLS 协议**: sing-box 的自定义 TLS 传输协议
- **Hysteria2**: 基于 QUIC 的高性能代理协议

## 注意事项

1. **公网 IP 检测**: 依赖外部服务,可能受网络环境影响
2. **ACME 证书**: 需要有效的域名和 DNS 配置
3. **端口权限**: 监听 443 等特权端口需要 root 权限或 CAP_NET_BIND_SERVICE
4. **防火墙**: 确保配置的端口在防火墙中开放

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

提交前请确保:
- 代码通过 `cargo fmt` 格式化
- 通过 `cargo clippy` 检查
- 所有测试通过 (`cargo test`)

## 许可证

查看项目根目录的 LICENSE 文件了解详情。
