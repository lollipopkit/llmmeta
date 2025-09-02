# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

LLM Meta 是一个自动抓取 models.dev API 数据并生成多语言 SDK 的工具。它从 models.dev 获取最新的 AI 模型信息，然后生成 Rust、Dart、Go、Python、TypeScript 等语言的 SDK 代码。

## 分支策略

- **main 分支**: 仅包含模板代码和生成工具，不包含生成的 SDK 代码
- **SDK 分支**: 通过 GitHub Actions 自动生成，每种语言对应一个分支：
  - `sdk/rust` - Rust SDK
  - `sdk/dart` - Dart SDK  
  - `sdk/go` - Go SDK
  - `sdk/python` - Python SDK
  - `sdk/typescript` - TypeScript SDK

## 常用命令

```bash
# 构建项目
cargo build --release

# 获取最新模型数据
./target/release/llmmeta fetch --output models.json

# 生成特定语言的SDK
./target/release/llmmeta generate --input models.json --lang rust --output output/rust
./target/release/llmmeta generate --input models.json --lang dart --output output/dart
./target/release/llmmeta generate --input models.json --lang go --output output/go
./target/release/llmmeta generate --input models.json --lang python --output output/python
./target/release/llmmeta generate --input models.json --lang typescript --output output/typescript

# 查看命令帮助
./target/release/llmmeta --help
./target/release/llmmeta fetch --help
./target/release/llmmeta generate --help

# 开发构建
cargo build

# 运行开发版本
./target/debug/llmmeta fetch --output test.json
./target/debug/llmmeta generate --input test.json --lang rust --output test-output/
```

## 代码架构

### 核心模块

- **`src/main.rs`**: CLI 入口点，使用 clap 处理命令行参数，支持 `fetch` 和 `generate` 两个子命令
- **`src/api.rs`**: 负责与 models.dev API 交互，包含数据获取、解析和存储逻辑
- **`src/models.rs`**: 定义数据结构，包括 Model、Provider、Modalities、Cost、Limit 等结构体
- **`src/codegen.rs`**: 代码生成引擎，使用 Handlebars 模板系统生成多语言代码

### 模板系统

- **`src/codegen/templates/`**: 包含各语言的 Handlebars 模板
  - `rust.rs` - Rust SDK 模板
  - `dart.rs` - Dart SDK 模板
  - `go.rs` - Go SDK 模板
  - `python.rs` - Python SDK 模板
  - `typescript.rs` - TypeScript SDK 模板

每个模板文件包含三个主要模板：
- `MODELS_TEMPLATE`: 模型数据和结构定义
- `LIB_TEMPLATE`/`INDEX_TEMPLATE`: 库入口和辅助函数
- 包管理文件模板 (Cargo.toml, pubspec.yaml, package.json 等)

### 数据流

1. **API 调用**: `api::fetch_models()` 从 models.dev 获取 JSON 数据
2. **数据解析**: JSON 被解析为 `HashMap<String, ProviderData>` 结构
3. **数据转换**: 将按提供商组织的数据转换为扁平的 `Vec<Model>` 列表
4. **代码生成**: `codegen::generate_sdk()` 使用 Handlebars 模板生成目标语言代码

### models.dev API 结构

API 返回的数据结构：
```
{
  "provider_name": {
    "id": "provider_id",
    "name": "Provider Name", 
    "api": "https://api.url",
    "models": {
      "model_id": {
        "name": "Model Name",
        "reasoning": true/false,
        "tool_call": true/false,
        "modalities": {
          "input": ["text", "image"],
          "output": ["text"]
        },
        "cost": {
          "input": 0.001,
          "output": 0.002
        },
        "limit": {
          "context": 128000,
          "output": 4096
        }
      }
    }
  }
}
```

## GitHub Actions 自动化

- **触发条件**: 每日 2AM UTC 自动运行，或手动触发，或 main 分支代码变更
- **工作流程**:
  1. 构建项目
  2. 获取最新模型数据
  3. 检查数据是否有变更
  4. 如有变更，生成所有语言的 SDK
  5. 将 models.json 提交到 main 分支
  6. 将各语言 SDK 强制推送到对应分支
  7. 创建新的 release

## 开发注意事项

- 在修改模板时，需要确保 Handlebars 语法正确，特别注意数组遍历和条件判断
- API 结构可能变化，需要相应更新 `src/models.rs` 中的数据结构定义
- 生成的测试文件应该被清理，不要提交到 main 分支
- 对于 Rust 代码，允许存在未使用的函数（这些是为生成的 SDK 准备的辅助函数）