# LLM Meta

[中文文档](README.zh.md)

LLM Meta fetches model metadata from <https://models.dev/api.json> and publishes generated SDK code for Rust, Dart, Go, Python, and TypeScript.

Use it when your application needs an up-to-date local catalog of LLM models, providers, modalities, context limits, pricing, reasoning support, function calling support, and open-weight status.

## Using the SDKs

Generated SDKs are committed to this repository under `sdks/`:

```text
sdks/rust
sdks/dart
sdks/go
sdks/python
sdks/typescript
```

You can vendor the SDK directory you need, copy it into your project, or depend on this repository through your package manager or Git tooling.

### Rust

Use `sdks/rust` as a Rust crate:

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

Use `sdks/python` as a local Python package:

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

Use `sdks/typescript` as a local package:

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

Use `sdks/go` as a Go module:

```sh
cd sdks/go
go test ./...
```

The generated Go SDK currently uses package `main`, so the simplest consumption path is to vendor or copy the generated files into your Go project and call helpers such as `GetAllModels`, `GetModelsByProvider`, and `GetModelsSortedByPrice`.

```go
models := GetModelsByModality("image")
fmt.Println(len(models))
```

### Dart

Use `sdks/dart` as a local Dart package:

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

## Common Queries

Every generated SDK exposes helpers for common model lookups:

- Get all models.
- Filter models by provider.
- Filter models by modality.
- Find function-calling models.
- Find reasoning-capable models.
- Find open-weight models.
- Sort models by price.

## Features

- Fetch the latest model data from models.dev.
- Generate SDKs for Rust, Dart, Go, Python, and TypeScript.
- Analyze and filter models by modality, function calling, reasoning support, open-weight status, provider, and price.
- Keep generated SDKs in `sdks/` through the GitHub Actions workflow.
- Run local checks through the included `Makefile`.

## Data Source

LLM Meta uses the models.dev API as its source of truth.

The dataset includes model information such as:

- Basic metadata: model name, release date, and knowledge cutoff.
- Capabilities: input and output modalities such as text, image, audio, video, and PDF.
- Limits: context length and output token limits.
- Pricing: input and output token pricing when available.
- Special capabilities: reasoning, function calling, and tool use.
- Open-weight status.
- Provider metadata.

## CLI Usage

Fetch the latest model data:

```sh
cargo run -- fetch --output models.json
```

Analyze local model data:

```sh
cargo run -- analyze --input models.json --group-by-provider
cargo run -- analyze --input models.json --modality image --function-calling
cargo run -- analyze --input models.json --open-source --sort-by-price
```

Generate an SDK:

```sh
cargo run -- generate --input models.json --lang rust --output sdks/rust
cargo run -- generate --input models.json --lang python --output sdks/python
```

Supported language values:

- `rust` or `rs`
- `dart`
- `go` or `golang`
- `python` or `py`
- `typescript` or `ts`

When model data or generated SDK files change, the workflow commits `models.json` and `sdks/` to `main` in a single commit.

## Development

Run the standard local checks:

```sh
make ci
```

Generate and verify all SDKs locally:

```sh
make verify-generated
```

Temporary generated files are written under `/private/tmp/llmmeta` by default. You can override this path:

```sh
make verify-generated TMP_DIR=/tmp/llmmeta
```
