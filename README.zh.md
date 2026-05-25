# LLM Meta

[English](README.md)

自动抓取 <https://models.dev/api.json> 中的模型信息，并发布对应的 Rust、Dart、Go、Python、TypeScript SDK 代码。

适用于应用需要在本地使用最新 LLM 模型目录的场景，包括提供商、模态、上下文长度、价格、推理能力、函数调用能力和开放权重状态。

## 使用 SDK

生成的 SDK 会提交到仓库的 `sdks/` 目录：

```text
sdks/rust
sdks/dart
sdks/go
sdks/python
sdks/typescript
```

你可以按需 vendor 对应目录、复制到项目中，或通过包管理器/Git 工具引用本仓库。

### Rust

把 `sdks/rust` 作为 Rust crate 使用：

```toml
[dependencies]
llm-models = { path = "sdks/rust" }
```

```rust
use llm_models::{get_function_calling_models, get_models_by_modality};

fn main() {
    let image_models = get_models_by_modality("image");
    let tool_models = get_function_calling_models();

    println!("image-capable models: {}", image_models.len());
    println!("function-calling models: {}", tool_models.len());
}
```

### Python

把 `sdks/python` 作为本地 Python 包安装：

```sh
pip install ./sdks/python
```

```python
import llm_models

image_models = llm_models.get_models_by_modality("image")
cheapest = llm_models.get_models_sorted_by_price()[0]

print(len(image_models))
print(cheapest.name, cheapest.provider.name)
```

### TypeScript

把 `sdks/typescript` 作为本地 package 使用：

```sh
npm install ./sdks/typescript
```

```ts
import { getModelsByProvider, getLargestContextModels } from "llm-models";

const openAIModels = getModelsByProvider("OpenAI");
const largestContext = getLargestContextModels(5);

console.log(openAIModels.length);
console.log(largestContext.map((model) => model.name));
```

### Go

把 `sdks/go` 作为 Go module 使用：

```sh
cd sdks/go
go test ./...
```

当前生成的 Go SDK 使用 `main` package，最直接的使用方式是把生成文件 vendor 或复制到你的 Go 项目，然后调用 `GetAllModels`、`GetModelsByProvider`、`GetModelsSortedByPrice` 等辅助函数。

```go
models := GetModelsByModality("image")
fmt.Println(len(models))
```

### Dart

把 `sdks/dart` 作为本地 Dart package 使用：

```yaml
dependencies:
  llm_models:
    path: sdks/dart
```

```dart
import 'package:llm_models/llm_models.dart';

void main() {
  final imageModels = getModelsByModality('image');
  final openSourceModels = getOpenSourceModels();

  print(imageModels.length);
  print(openSourceModels.length);
}
```

## 常用查询

每个生成的 SDK 都提供常见模型查询辅助函数：

- 获取全部模型。
- 按提供商筛选模型。
- 按模态筛选模型。
- 查找支持函数调用的模型。
- 查找支持推理的模型。
- 查找开放权重模型。
- 按价格排序模型。

## 功能特性

- 从 models.dev 获取最新模型数据。
- 支持生成 Rust、Dart、Go、Python、TypeScript SDK。
- 支持按模态、函数调用、推理能力、开放权重状态、提供商和价格分析筛选模型。
- GitHub Actions 会把生成的 SDK 写入 `sdks/`。
- 提供 `Makefile` 统一本地检查和生成验证命令。

## 数据源信息

LLM Meta 使用 models.dev API 作为数据源。

模型信息包括：

- 基本信息：模型名称、发布日期、知识截止日期。
- 能力特性：输入/输出模态，如文本、图像、音频、视频、PDF。
- 技术规格：上下文长度、输出 token 限制。
- 定价信息：可用时包含输入/输出 token 价格。
- 特殊能力：推理能力、函数调用、工具使用。
- 开放权重状态。
- 提供商信息。

## CLI 使用方式

获取最新模型数据：

```sh
cargo run -- fetch --output models.json
```

分析本地模型数据：

```sh
cargo run -- analyze --input models.json --group-by-provider
cargo run -- analyze --input models.json --modality image --function-calling
cargo run -- analyze --input models.json --open-source --sort-by-price
```

生成 SDK：

```sh
cargo run -- generate --input models.json --lang rust --output sdks/rust
cargo run -- generate --input models.json --lang python --output sdks/python
```

支持的语言参数：

- `rust` 或 `rs`
- `dart`
- `go` 或 `golang`
- `python` 或 `py`
- `typescript` 或 `ts`

当模型数据或生成的 SDK 文件发生变化时，workflow 会把 `models.json` 和 `sdks/` 合并为一个 commit 提交到 `main`。

## 开发

运行标准本地检查：

```sh
make ci
```

本地生成并验证全部 SDK：

```sh
make verify-generated
```

临时生成文件默认写入 `/private/tmp/llmmeta`。可以覆盖该路径：

```sh
make verify-generated TMP_DIR=/tmp/llmmeta
```
