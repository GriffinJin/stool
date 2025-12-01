# Synapse CLI - Smart Development Tools

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Synapse CLI 是一个强大的开发工具集，专为提高开发效率而设计。它提供了 Git 仓库批量管理、版本号替换、数据库文件处理等功能，大大提高了开发者的工作效率。

## ✨ 特性

### 🔧 仓库管理 (Repo)
- **批量拉取** - 并发拉取当前目录下所有 Git 仓库
- **仓库列表** - 显示仓库状态、分支信息、提交差异等
- **分支切换** - 批量切换仓库分支，支持创建新分支
- **仓库清理** - 批量重置和清理仓库工作区
- **克隆脚本生成** - 生成批量克隆脚本，支持 HTTP/SSH 协议

### 📝 版本管理 (Version)
- **版本替换** - 智能查找并替换项目中的版本号

### 🗄️ 数据库工具 (DB)
- **清理 ID 字段** - 移除 SQL INSERT 语句中的 id 字段及对应值

## 🚀 安装

### 从源码构建

```bash
git clone https://github.com/your-username/synapse-cli.git
cd synapse-cli
cargo build --release
```

构建完成后，可执行文件位于 `target/release/synapse`。

### 添加到 PATH

```bash
# 将可执行文件复制到系统路径
sudo cp target/release/synapse /usr/local/bin/

# 或者添加到你的 shell 配置文件
echo 'export PATH="$PATH:/path/to/synapse-cli/target/release"' >> ~/.bashrc
```

## 📖 使用指南

### 仓库管理命令

#### 查看仓库列表
```bash
# 显示当前目录下所有 Git 仓库的状态
synapse repo ls

# 同时获取远程仓库信息（会执行 git fetch）
synapse repo ls --fetch
```

#### 批量拉取更新
```bash
# 并发拉取所有仓库的最新代码
synapse repo pull
```

#### 批量切换分支
```bash
# 切换到指定分支
synapse repo switch main

# 强制切换（如果分支不存在则创建）
synapse repo switch --force feature-branch
```

#### 批量清理仓库
```bash
# 重置所有仓库到 HEAD 并清理未跟踪文件
synapse repo clean
```

#### 生成克隆脚本
```bash
# 输出克隆命令到控制台
synapse repo genclone

# 保存克隆脚本到文件
synapse repo genclone -f --path ./clone_all.sh

# 使用 SSH 协议
synapse repo genclone -t ssh -f
```

### 版本管理命令

#### 替换版本号
```bash
# 在当前目录下查找并替换版本号
synapse version replace "1.0.0" "1.1.0"
```

### 数据库工具命令

#### 清理 SQL 文件中的 ID 字段
```bash
# 移除 INSERT 语句中的 id 字段及对应值（会生成 .bak 备份文件）
synapse db rmid ./data.sql
```

## 🏗️ 项目结构

```
src/
├── lib.rs              # 库入口，模块导出
├── main.rs             # 主程序入口
├── cli/                # 命令行接口
│   ├── mod.rs          # CLI 模块入口
│   └── commands.rs     # 命令定义
├── repo/               # Git 仓库操作相关
│   ├── mod.rs          # Repo 模块入口
│   ├── repo.rs         # 仓库发现和信息获取
│   ├── operations.rs   # Git 操作（pull, fetch, switch 等）
│   └── clone.rs        # 克隆相关功能
├── version/            # 版本管理
│   ├── mod.rs          # 版本模块入口
│   └── replace.rs      # 版本替换功能
├── db/                 # 数据库工具
│   ├── mod.rs          # DB 模块入口
│   └── rmid.rs         # 清理 SQL ID 字段
└── utils/              # 工具模块
    ├── mod.rs          # 工具模块入口
    ├── command.rs      # 命令执行工具
    └── parallel.rs     # 并发执行框架
```

## 🔧 依赖要求

- **Rust** 1.70 或更高版本
- **Git** - 系统需要安装 Git
- **ripgrep (rg)** - 用于文件搜索（版本替换功能）
- **sd** - 用于文本替换（版本替换功能）

### 安装依赖工具

```bash
# macOS
brew install ripgrep sd

# Ubuntu/Debian
sudo apt install ripgrep
cargo install sd

# 其他系统请参考相应工具的官方文档
```

## 🤝 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 开发指南

```bash
# 克隆仓库
git clone https://github.com/your-username/synapse-cli.git
cd synapse-cli

# 运行测试
cargo test

# 检查代码格式
cargo fmt --check

# 运行 linter
cargo clippy
```

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [ripgrep](https://github.com/BurntSushi/ripgrep) - 快速文本搜索
- [sd](https://github.com/chmln/sd) - 直观的查找和替换工具

## 📞 支持

如果你遇到问题或有建议，请：

1. 查看 [Issues](https://github.com/your-username/synapse-cli/issues) 页面
2. 创建新的 Issue 描述问题
3. 或者直接提交 Pull Request

---

**Synapse CLI** - 让开发工作变得简单高效！ 🚀