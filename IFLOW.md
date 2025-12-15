# ezsingbox - sing-box 自动化配置生成器

## 项目概述

**ezsingbox** 是一个用 Rust 编写的 sing-box 配置自动化生成工具。它提供了简单易用的命令行工具和 API 来自动生成和管理 sing-box 的服务端和客户端配置,特别针对 **AnyTLS**、**Hysteria2** 和 **TUIC** 三种协议。

### 核心功能

- 🚀 **自动化配置生成**: 通过环境变量或 Builder 模式快速生成 sing-box 配置
- 🔐 **用户管理**: 自动生成用户密码和 UUID,支持自定义凭证
- 🌐 **公网 IP 自动检测**: 自动从多个服务获取公网 IP
- 🔧 **TLS 自动化**: 支持 ACME 自动证书申请
- 📦 **域名生成**: 自动生成 sslip.io/nip.io 域名用于 TLS
- 🎯 **多协议支持**: AnyTLS、Hysteria2、TUIC 协议
- 📡 **订阅服务**: 内置 HTTP 订阅服务器,支持 sing-box 远程配置导入
- 🐳 **Docker 支持**: 提供多架构 Docker 镜像

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
  - `tiny_http`: 轻量级 HTTP 服务器(用于订阅服务)

## 项目结构

```
ezsingbox/
├── Cargo.toml                  # 项目配置和依赖
├── Cargo.lock                  # 依赖锁定文件
├── Dockerfile                  # Docker 多架构构建配置
├── config.json                 # 示例服务端配置
├── client.json                 # 示例客户端配置
├── .github/
│   └── workflows/
│       ├── ci.yml              # CI 工作流(检查/测试/构建)
│       └── docker-build.yml    # Docker 镜像构建工作流
└── src/
    ├── main.rs                 # 主程序入口(CLI 工具)
    ├── mod.rs                  # 模块声明
    ├── autoconfig/             # 自动化配置生成模块
    │   ├── mod.rs              # 模块导出
    │   ├── autoByDefault.rs    # 多协议统一构建器
    │   ├── autoanytls.rs       # AnyTLS 配置生成器
    │   ├── autohy2.rs          # Hysteria2 配置生成器
    │   ├── autotuic.rs         # TUIC 配置生成器
    │   └── tools.rs            # 通用工具函数
    ├── dns/
    │   └── dns.rs              # DNS 配置
    └── singboxconfig/          # sing-box 配置数据模型
        ├── mod.rs              # 模块声明
        ├── lib.rs              # 库入口
        ├── full.rs             # 完整配置结构
        ├── inbound/            # 入站配置
        │   ├── mod.rs
        │   ├── anytls.rs       # AnyTLS 入站
        │   ├── hysteria2.rs    # Hysteria2 入站
        │   └── tuic.rs         # TUIC 入站
        ├── outbound/           # 出站配置
        │   ├── mod.rs
        │   └── anytls.rs       # AnyTLS 出站
        ├── shared/             # 共享配置
        │   ├── mod.rs
        │   ├── tls.rs          # TLS 配置
        │   ├── dns01_challenge.rs  # DNS-01 挑战
        │   ├── listen.rs       # 监听配置
        │   ├── dial.rs         # 拨号配置
        │   ├── multiplex.rs    # 多路复用
        │   └── v2ray.rs        # V2Ray 传输
        └── types/              # 自定义类型
            ├── mod.rs
            ├── user.rs         # 用户类型
            ├── duration.rs     # 时长类型
            ├── domain_strategy.rs      # 域名策略
            ├── network_strategy.rs     # 网络策略
            ├── routing_mark.rs # 路由标记
            └── string_or_array.rs      # 字符串或数组
```

## 构建和运行

### 开发环境要求

- Rust 工具链 1.85+ (推荐使用 rustup)
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

# 运行程序(生成配置)
cargo run -- generate

# 运行程序(生成配置并启动 sing-box)
cargo run -- run

# 运行订阅服务
cargo run -- serve

# Release 版本运行
cargo run --release -- generate
```

### 代码检查和格式化

```bash
# 格式化代码
cargo fmt

# Lint 检查
cargo clippy --all-features

# 完整检查(推荐在提交前运行)
cargo fmt && cargo clippy --all-features && cargo test --all-features
```

## CLI 使用方法

ezsingbox 提供三个主要命令:

### 1. generate - 生成配置文件

生成 sing-box 服务端配置文件(默认命令)。

```bash
ezsingbox generate
# 或直接
ezsingbox
```

**关键环境变量**:

```bash
# 服务端配置
export EZ_CONFIG_PATH="./config.json"        # 配置文件路径
export EZ_PUBLIC_IP="203.0.113.1"            # 公网 IP(可选,自动检测)
export EZ_DOMAIN="example.com"               # 域名(可选,自动生成 sslip.io)
export EZ_LOG_LEVEL="info"                   # 日志级别

# 协议开关
export EZ_ENABLE_ANYTLS=true                 # 启用 AnyTLS(默认 true)
export EZ_ENABLE_HYSTERIA2=true              # 启用 Hysteria2(默认 true)
export EZ_ENABLE_TUIC=true                   # 启用 TUIC(默认 true)

# 端口配置
export EZ_ANYTLS_PORT=443                    # AnyTLS 端口(默认 443)
export EZ_HYSTERIA2_PORT=2053                # Hysteria2 端口(默认 2053)
export EZ_TUIC_PORT=2083                     # TUIC 端口(默认 2083)

# 用户凭证
export EZ_USER="myuser"                      # 用户名(默认 "default")
export EZ_PASSWORD="mypassword"              # 密码(可选,自动生成)

# Hysteria2 特定配置
export EZ_HY2_OBFS=true                      # 启用混淆
export EZ_HY2_UP_MBPS=100                    # 上行带宽(Mbps)
export EZ_HY2_DOWN_MBPS=100                  # 下行带宽(Mbps)

# TUIC 特定配置
export EZ_TUIC_CC="bbr"                      # 拥塞控制算法(bbr/cubic/new_reno)

# 输出控制
export EZ_PRINT_CONFIG=true                  # 打印配置内容(默认 true)
export EZ_PRINT_DETAILS=true                 # 打印详细信息(默认 true)

# 客户端配置生成
export EZ_CLIENT_CONFIG_PATH="./client.json" # 客户端配置文件路径
export EZ_CLIENT_PROTOCOL="anytls"           # 客户端使用的协议
export EZ_CLIENT_USER="myuser"               # 客户端使用的用户
export EZ_CLIENT_MIXED_LISTEN="127.0.0.1"    # 客户端监听地址
export EZ_CLIENT_MIXED_PORT=7890             # 客户端监听端口
```

### 2. run - 生成配置并启动 sing-box

生成配置文件后直接启动 sing-box 服务。

```bash
ezsingbox run
```

**额外环境变量**:

```bash
export SING_BOX_BIN="sing-box"               # sing-box 二进制路径
```

### 3. serve - 启动订阅服务

启动 HTTP 订阅服务器,提供客户端配置订阅。

```bash
ezsingbox serve
```

**订阅服务环境变量**:

```bash
export EZ_SUBSCRIBE_LISTEN="0.0.0.0:8080"    # 监听地址(默认 0.0.0.0:8080)
export EZ_SUBSCRIBE_PATH="/config.json"      # 订阅路径(默认 /config.json)
export EZ_SUBSCRIBE_PUBLIC_URL="http://..."  # 公网访问 URL(可选)
export EZ_SUBSCRIBE_NAME="ezsingbox"         # 配置名称
export EZ_SUBSCRIBE_BASIC_USER="admin"       # HTTP Basic 认证用户名(可选)
export EZ_SUBSCRIBE_BASIC_PASS="password"    # HTTP Basic 认证密码(可选)
```

订阅服务会输出:
- 订阅链接: HTTP URL
- URI 链接: `sing-box://import-remote-profile?url=...` 格式

## Docker 使用

### 使用预构建镜像

```bash
# 拉取镜像
docker pull ghcr.io/laomeifun/ezsingbox:latest

# 运行(生成配置并启动)
docker run -d \
  --name ezsingbox \
  -p 443:443 \
  -p 2053:2053/udp \
  -p 2083:2083/udp \
  -e EZ_DOMAIN="example.com" \
  -e EZ_USER="myuser" \
  -e EZ_PASSWORD="mypassword" \
  ghcr.io/laomeifun/ezsingbox:latest run

# 运行订阅服务
docker run -d \
  --name ezsingbox-subscribe \
  -p 8080:8080 \
  -e EZ_DOMAIN="example.com" \
  -e EZ_USER="myuser" \
  -e EZ_PASSWORD="mypassword" \
  ghcr.io/laomeifun/ezsingbox:latest serve
```

### 本地构建镜像

```bash
# 构建多架构镜像
docker buildx build --platform linux/amd64,linux/arm64 -t ezsingbox:local .

# 构建单架构镜像
docker build -t ezsingbox:local .
```

## 代码开发指南

### 代码风格约定

- 遵循 Rust 标准命名约定
- 使用 `cargo fmt` 格式化代码
- 通过 `cargo clippy` 检查
- 为公共 API 编写文档注释 (`///`)
- 为模块编写模块级文档 (`//!`)
- 错误处理使用 `Result<T, E>` 模式

### 模块组织原则

- **autoconfig**: 高级自动化配置生成 API,使用 Builder 模式
- **singboxconfig**: 底层 sing-box 配置数据结构,直接映射 JSON
- **main.rs**: CLI 工具实现,环境变量解析和命令分发

### 添加新协议支持

1. 在 `src/singboxconfig/inbound/` 添加新协议的入站配置结构
2. 在 `src/autoconfig/` 添加对应的自动配置生成器
3. 在 `src/autoconfig/autoByDefault.rs` 的 `MultiProtocolBuilder` 中集成
4. 在 `src/main.rs` 中添加环境变量支持和命令行参数

### 测试

```bash
# 运行所有测试
cargo test --all-features

# 运行特定模块测试
cargo test --package ezsingbox --lib autoconfig

# 显示测试输出
cargo test -- --nocapture
```

## CI/CD 工作流

### GitHub Actions 配置

#### 1. CI 工作流 (`.github/workflows/ci.yml`)

**触发条件**: 推送到 `main`/`master`/`develop` 分支或 PR

**任务**:
- **Check**: `cargo check --all-features`
- **Test**: `cargo test --all-features`
- **Build**: `cargo build --release`

#### 2. Docker 构建工作流 (`.github/workflows/docker-build.yml`)

**触发条件**: 推送到主分支或手动触发

**功能**:
- 多架构构建 (amd64, arm64)
- 推送到 GitHub Container Registry
- 基于官方 sing-box 镜像构建

### 本地验证 CI 流程

```bash
# 模拟 CI 检查
cargo check --all-features
cargo test --all-features
cargo build --release

# 检查代码质量
cargo fmt -- --check
cargo clippy --all-features -- -D warnings
```

## 架构设计

### 配置生成流程

```
环境变量 → Builder → 协议配置 → JSON 序列化 → 文件输出
   ↓
自动检测 IP → 生成域名 → ACME 配置 → TLS 配置
   ↓
用户管理 → 生成密码/UUID → 用户列表
```

### 核心组件

1. **MultiProtocolBuilder**: 统一的多协议配置构建器
   - 自动检测公网 IP
   - 自动生成 sslip.io 域名
   - 支持多用户管理
   - 协议独立配置

2. **SingBoxConfig**: 完整的 sing-box 配置结构
   - 服务端配置模板 (`server_default`)
   - 客户端配置模板 (`client_default`)
   - JSON 序列化支持

3. **订阅服务**: 轻量级 HTTP 服务器
   - 动态生成客户端配置
   - 支持 HTTP Basic 认证
   - sing-box URI scheme 支持

## 常见使用场景

### 场景 1: 快速部署单协议服务

```bash
# 只启用 Hysteria2
export EZ_ENABLE_ANYTLS=false
export EZ_ENABLE_TUIC=false
export EZ_ENABLE_HYSTERIA2=true
export EZ_HYSTERIA2_PORT=443
export EZ_HY2_UP_MBPS=500
export EZ_HY2_DOWN_MBPS=500

ezsingbox run
```

### 场景 2: 多用户配置

通过代码使用 Builder API:

```rust
use ezsingbox::autoconfig::MultiProtocolBuilder;

let result = MultiProtocolBuilder::new()
    .domain("example.com")
    .enable_all()
    .add_user("user1")
    .add_user_with_password("user2", "custom_pass")
    .add_user("user3")
    .build()?;
```

### 场景 3: 订阅服务部署

```bash
# 启动订阅服务
export EZ_DOMAIN="vpn.example.com"
export EZ_SUBSCRIBE_LISTEN="0.0.0.0:8443"
export EZ_SUBSCRIBE_BASIC_USER="admin"
export EZ_SUBSCRIBE_BASIC_PASS="secure_password"

ezsingbox serve

# 客户端订阅链接
# http://vpn.example.com:8443/config.json
```

## 注意事项

1. **公网 IP 检测**: 依赖外部服务(ipify.org, api.ip.sb 等),可能受网络环境影响
2. **ACME 证书**: 需要有效的域名和 DNS 配置,端口 80/443 必须可访问
3. **端口权限**: 监听 443 等特权端口需要 root 权限或 `CAP_NET_BIND_SERVICE`
4. **防火墙**: 确保配置的端口在防火墙中开放(TCP/UDP)
5. **Docker 网络**: 使用 Docker 时注意端口映射和网络模式

## 相关资源

- **sing-box 官方文档**: https://sing-box.sagernet.org/
- **Hysteria2 协议**: https://v2.hysteria.network/
- **TUIC 协议**: https://github.com/EAimTY/tuic
- **项目仓库**: https://github.com/laomeifun/ezsingbox

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

**提交前检查清单**:
- [ ] 代码通过 `cargo fmt` 格式化
- [ ] 通过 `cargo clippy --all-features` 检查
- [ ] 所有测试通过 (`cargo test --all-features`)
- [ ] 更新相关文档
- [ ] 提交信息遵循约定式提交规范

## 开发环境配置

### 推荐的 VSCode 扩展

- `rust-analyzer`: Rust 语言服务器
- `crates`: Cargo.toml 依赖管理
- `Even Better TOML`: TOML 语法高亮

### 推荐的 Rust 工具

```bash
# 安装常用工具
cargo install cargo-watch    # 文件变化自动重新编译
cargo install cargo-edit     # 命令行管理依赖
cargo install cargo-outdated # 检查过期依赖

# 使用 cargo-watch 自动重新编译
cargo watch -x check -x test
```

## 许可证

查看项目根目录的 LICENSE 文件了解详情。

---

**系统环境**: Arch Linux  
**Shell**: zsh  
**默认语言**: 中文
