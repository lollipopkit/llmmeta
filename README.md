# LLM Meta

自动抓取 <https://models.dev/api.json> 中的模型信息，并生成对应的 rust、dart、golang、python、typescript 模板代码。

便于快速获取模型信息。

## 数据源信息

### 支持的模型提供商

基于 models.dev API 数据，支持以下主要 AI 模型提供商：

- **OpenAI**: GPT-4o, GPT-4 系列, GPT-3.5, o1 系列
- **Anthropic**: Claude 3.5 Sonnet, Claude 3 系列, Claude Haiku
- **Google**: Gemini Pro/Flash, Vertex AI 系列
- **Meta**: Llama 3.1/3.2 系列
- **Mistral AI**: Mistral Large, Codestral, Pixtral
- **xAI**: Grok 系列
- **DeepSeek**: DeepSeek Coder, DeepSeek Chat
- **Zhipu AI**: GLM 4 系列
- **Alibaba**: Qwen 系列
- **其他**: Cohere, Together AI, Perplexity, Fireworks 等

### 模型信息包含

每个模型的详细信息包括：

- **基本信息**: 模型名称、发布日期、知识截止日期
- **能力特性**: 输入/输出模态（文本、图像、音频、视频）
- **技术规格**: 上下文长度、输出令牌限制
- **定价信息**: 输入/输出令牌价格
- **特殊能力**: 推理能力、函数调用、工具使用
- **开源状态**: 是否开放权重

## 功能特性

- 🔄 自动同步 models.dev 最新模型数据，使用 GitHub Actions 自动更新
- 📊 支持多种编程语言的代码生成
- 🏷️ 结构化模型信息提取
- 💰 价格信息对比分析
- 🎯 多模态能力检索
