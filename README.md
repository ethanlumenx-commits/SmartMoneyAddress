# Smart Money - Solana 链上交易追踪机器人

## 📖 项目简介

Smart Money 是一个基于 Rust 开发的 Solana 区块链智能资金追踪系统。该系统通过 Helius API 和 WebSocket 实时监听链上交易活动，特别关注大额交易（"聪明钱"）和代币交换（Swap）操作，并将数据存储到 PostgreSQL 数据库中进行分析和追踪。

## ✨ 核心功能

### 1. **Helius API 集成**
- 获取指定地址的交易签名历史
- 解析增强型交易数据（Enhanced Transactions）
- 识别和过滤 Swap 交易事件
- 支持批量交易数据处理

### 2. **WebSocket 实时监听**
- 实时订阅 Solana 账户变更
- 监听特定地址的链上活动
- 即时接收交易通知

### 3. **数据持久化**
- PostgreSQL 数据库存储
- 三种数据模型：
  - `raw_addresses`: 原始地址数据
  - `helius_json`: Helius API 返回的完整交易 JSON
  - `swaps`: 提取的 Swap 交易记录
- 支持批量插入和查询操作

### 4. **配置管理**
- 环境变量配置（`.env` 文件）
- 支持多环境配置（生产/测试）
- Helius API 密钥管理

### 5. **日志系统**
- 基于 `tracing` 的结构化日志
- 可配置的日志级别
- 环境变量控制日志输出

## 🏗️ 项目结构

```
smart_money/
├── src/
│   ├── config.rs          # 配置管理模块
│   ├── logger.rs          # 日志初始化模块
│   ├── lib.rs             # 库入口
│   ├── main.rs            # 程序入口
│   ├── database/          # 数据库连接
│   │   ├── mod.rs
│   │   └── connect.rs     # PostgreSQL 连接池
│   ├── dbmodel/           # 数据模型
│   │   ├── mod.rs
│   │   ├── raw_addresses.rs
│   │   ├── helius_json.rs
│   │   └── swaps.rs       # Swap 交易模型
│   ├── repo/              # 数据访问层 (SQL)
│   │   ├── mod.rs
│   │   ├── raw_addresses_repo.rs
│   │   ├── helius_json_repo.rs
│   │   └── swaps_repo.rs  # Swap 数据操作
│   ├── service/           # 业务逻辑层
│   │   ├── mod.rs
│   │   ├── raw_addresses_seriver.rs
│   │   ├── helius_json_seriver.rs
│   │   ├── swaps_seriver.rs    # Swap 业务逻辑
│   │   ├── address_read.rs     # 地址读取服务
│   │   └── websockt_read.rs    # WebSocket 监听服务
│   └── helius/            # Helius API 客户端
│       ├── mod.rs
│       ├── client.rs      # 基础 API 客户端
│       └── enhanced.rs    # 增强型 API 客户端
├── examples/              # 示例代码
│   ├── heliusclient.rs    # Helius 客户端使用示例
│   ├── heliusenhanced.rs  # 增强 API 使用示例
│   ├── heliusswap.rs      # Swap 交易查询示例
│   ├── dbconnect.rs       # 数据库连接示例
│   └── ...
├── Cargo.toml
├── .env                   # 环境配置文件
└── README.md
```

## 🚀 快速开始

### 前置要求

- Rust 1.75+ (Edition 2024)
- PostgreSQL 数据库
- Helius API Key（从 [Helius](https://www.helius.dev/) 获取）

### 安装步骤

1. **克隆项目**
```bash
git clone <repository-url>
cd smart_money
```

2. **配置环境变量**

创建 `.env` 文件并填写配置：
```env
# PostgreSQL 数据库配置
PG_HOST=localhost
PG_USER=postgres
PG_PASSWORD=your_password
PG_DB=smart_money
PG_PORT=5432

# Helius API 配置
HELIUS_API_URL=https://mainnet.helius-rpc.com/?api-key=YOUR_API_KEY
HELIUS_API_URL_BETA=https://testnet.helius-rpc.com/?api-key=YOUR_API_KEY
HELIUS_API_KEY=YOUR_API_KEY
HELIUS_ENHANCED_API_URL=https://api.helius.xyz/v0/transactions/?api-key=YOUR_API_KEY
HELIUS_WEBSOCKS_URL_KEY=wss://mainnet.helius-rpc.com/?api-key=YOUR_API_KEY
```

3. **运行示例**
```bash
# 测试 Helius 客户端
cargo run --example heliusclient

# 测试增强 API
cargo run --example heliusenhanced

# 测试 Swap 交易查询
cargo run --example heliusswap

# 测试数据库连接
cargo run --example dbconnect
```

4. **构建项目**
```bash
cargo build
```

## 📦 依赖说明

| 依赖 | 版本 | 用途 |
|------|------|------|
| tokio | 1.0 | 异步运行时 |
| reqwest | 0.11 | HTTP 客户端 |
| sqlx | 0.7 | PostgreSQL 数据库驱动 |
| serde/serde_json | 1.0 | JSON 序列化/反序列化 |
| tokio-tungstenite | 0.21 | WebSocket 客户端 |
| tracing | 0.1 | 结构化日志 |
| config | 0.14 | 配置管理 |
| dotenv | 0.15 | 环境变量加载 |
| chrono | 0.4 | 时间处理 |
| bigdecimal | 0.3 | 高精度数值计算 |
| anyhow | 1.0 | 错误处理 |

## 🔧 核心模块说明

### Helius 客户端 (`src/helius/`)

**基础客户端** (`client.rs`)
- `get_signatures_for_address()`: 获取地址的交易签名列表

**增强客户端** (`enhanced.rs`)
- `get_transactions_for_signatures()`: 获取详细的增强型交易数据
- 自动解析 Swap、Transfer 等事件类型

### 服务层 (`src/service/`)

**SwapsService** (`swaps_seriver.rs`)
- `filter_swaps()`: 从交易列表中过滤出 Swap 交易
- `insert()` / `batch_insert()`: 将 Swap 数据存入数据库
- `find_by_signature()`: 根据签名查询 Swap 记录

**WebSocketService** (`websockt_read.rs`)
- `start()`: 启动 WebSocket 监听，订阅账户变更

### 数据模型 (`src/dbmodel/`)

- `SwapModel`: Swap 交易数据结构
- `HeliusJson`: Helius API 完整响应结构
- `RawAddresses`: 原始地址数据

## 📝 使用示例

### 查询地址的 Swap 交易

```rust
use smart_money::config::load_config;
use smart_money::helius::enhanced::EnhancedClient;
use smart_money::service::swaps_seriver::SwapsService;

#[tokio::main]
async fn main() {
    let config = load_config();
    let client = EnhancedClient::new(config.helius_enhanced_api_url);
    
    // 获取交易签名
    let signatures = vec!["signature1".to_string(), "signature2".to_string()];
    
    // 获取增强交易数据
    let transactions = client.get_transactions_for_signatures(signatures).await.unwrap();
    
    // 过滤出 Swap 交易
    let swaps = SwapsService::filter_swaps(transactions);
    
    println!("Found {} swap transactions", swaps.len());
}
```

### 监听账户实时变动

```rust
use smart_money::service::websockt_read::WebSocketService;

#[tokio::main]
async fn main() {
    // 监听指定地址的链上活动
    WebSocketService::start("675kPX9k5ZBu4h3d9MBUDNZA9HPc5xCi21QZ9vZJTkCy")
        .await
        .unwrap();
}
```

## 🎯 应用场景

1. **聪明钱追踪**: 监控知名交易者/机构的链上活动
2. **新币发现**: 实时检测新的代币交换行为
3. **套利机会**: 捕捉跨 DEX 的价格差异
4. **风险控制**: 监控大额资金流动
5. **数据分析**: 积累历史交易数据进行统计分析

## ⚠️ 注意事项

- Helius API 有速率限制，请合理使用
- WebSocket 连接需要稳定的网络环境
- 建议在生产环境使用连接池和重试机制
- 数据库表结构需要先执行 SQL 迁移脚本

## 📄 License

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

**提示**: 本项目仍处于开发阶段，部分功能可能需要进一步完善。如有问题，请查看 `examples/` 目录中的示例代码获取更多使用细节。
