use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::Path;

use crate::models::{Model, Provider};

mod templates;

/// 支持的编程语言
#[derive(Debug, Clone)]
pub enum Language {
    Rust,
    Dart,
    Go,
    Python,
    TypeScript,
}

#[derive(Debug, Serialize)]
struct CodegenModel<'a> {
    id: &'a str,
    name: &'a str,
    description: Option<&'a str>,
    context_length: Option<u32>,
    output_length: Option<u32>,
    input_cost: Option<f64>,
    output_cost: Option<f64>,
    release_date: Option<&'a str>,
    knowledge_cutoff: Option<&'a str>,
    modalities: Vec<&'a str>,
    reasoning: Option<bool>,
    function_calling: Option<bool>,
    tool_use: Option<bool>,
    open_weight: Option<bool>,
    provider: &'a Provider,
}

impl Language {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Language::Rust),
            "dart" => Ok(Language::Dart),
            "go" | "golang" => Ok(Language::Go),
            "python" | "py" => Ok(Language::Python),
            "typescript" | "ts" => Ok(Language::TypeScript),
            _ => Err(anyhow!("Unsupported language: {}", s)),
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::Dart => "dart",
            Language::Go => "go",
            Language::Python => "py",
            Language::TypeScript => "ts",
        }
    }

    pub fn package_file(&self) -> Option<&'static str> {
        match self {
            Language::Rust => Some("Cargo.toml"),
            Language::Dart => Some("pubspec.yaml"),
            Language::Go => Some("go.mod"),
            Language::Python => Some("pyproject.toml"),
            Language::TypeScript => Some("package.json"),
        }
    }
}

/// 生成指定语言的 SDK
pub fn generate_sdk(models: &[Model], lang: &str, output_dir: &str) -> Result<()> {
    let language = Language::from_str(lang)?;
    let output_path = Path::new(output_dir);

    println!(
        "正在生成 {} 语言的 SDK，文件扩展名: .{}",
        lang,
        language.file_extension()
    );

    // 创建输出目录
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }

    // 创建 Handlebars 实例
    let mut handlebars = Handlebars::new();

    // 注册模板
    match language {
        Language::Rust => {
            handlebars.register_template_string("models", templates::rust::MODELS_TEMPLATE)?;
            handlebars.register_template_string("lib", templates::rust::LIB_TEMPLATE)?;
            handlebars.register_template_string("cargo", templates::rust::CARGO_TEMPLATE)?;
        }
        Language::Dart => {
            handlebars.register_template_string("models", templates::dart::MODELS_TEMPLATE)?;
            handlebars.register_template_string("lib", templates::dart::LIB_TEMPLATE)?;
            handlebars.register_template_string("pubspec", templates::dart::PUBSPEC_TEMPLATE)?;
        }
        Language::Go => {
            handlebars.register_template_string("models", templates::go::MODELS_TEMPLATE)?;
            handlebars.register_template_string("main", templates::go::MAIN_TEMPLATE)?;
            handlebars.register_template_string("gomod", templates::go::GOMOD_TEMPLATE)?;
        }
        Language::Python => {
            handlebars.register_template_string("models", templates::python::MODELS_TEMPLATE)?;
            handlebars.register_template_string("init", templates::python::INIT_TEMPLATE)?;
            handlebars
                .register_template_string("pyproject", templates::python::PYPROJECT_TEMPLATE)?;
        }
        Language::TypeScript => {
            handlebars
                .register_template_string("models", templates::typescript::MODELS_TEMPLATE)?;
            handlebars.register_template_string("index", templates::typescript::INDEX_TEMPLATE)?;
            handlebars
                .register_template_string("package", templates::typescript::PACKAGE_TEMPLATE)?;
            handlebars
                .register_template_string("tsconfig", templates::typescript::TSCONFIG_TEMPLATE)?;
        }
    }

    let codegen_models = normalize_models(models);
    let models_json = serde_json::to_string_pretty(&codegen_models)?;
    let models_json_literal = serde_json::to_string(&models_json)?;
    let models_json_literal_dart = models_json_literal.replace('$', r"\$");

    // 准备模板数据
    let template_data = json!({
        "models": codegen_models,
        "models_json_literal": models_json_literal,
        "models_json_literal_dart": models_json_literal_dart,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "model_count": models.len(),
        "providers": crate::api::group_models_by_provider(models).keys().collect::<Vec<_>>()
    });

    // 生成代码文件
    generate_code_files(&handlebars, &language, output_path, &template_data)?;

    // 生成包配置文件
    if let Some(package_file) = language.package_file() {
        generate_package_file(
            &handlebars,
            &language,
            output_path,
            package_file,
            &template_data,
        )?;
    }

    Ok(())
}

fn normalize_models(models: &[Model]) -> Vec<CodegenModel<'_>> {
    models
        .iter()
        .map(|model| {
            let mut modalities = Vec::new();
            if let Some(model_modalities) = &model.modalities {
                if let Some(input) = &model_modalities.input {
                    extend_unique(&mut modalities, input);
                }
                if let Some(output) = &model_modalities.output {
                    extend_unique(&mut modalities, output);
                }
            }

            CodegenModel {
                id: &model.id,
                name: &model.name,
                description: None,
                context_length: model.get_context_length(),
                output_length: model.get_output_length(),
                input_cost: model.cost.as_ref().and_then(|cost| cost.input),
                output_cost: model.cost.as_ref().and_then(|cost| cost.output),
                release_date: model.release_date.as_deref(),
                knowledge_cutoff: model.knowledge.as_deref(),
                modalities,
                reasoning: model.reasoning,
                function_calling: model.tool_call,
                tool_use: model.tool_call,
                open_weight: model.open_weights,
                provider: &model.provider,
            }
        })
        .collect()
}

fn extend_unique<'a>(target: &mut Vec<&'a str>, source: &'a [String]) {
    for item in source {
        if !target.contains(&item.as_str()) {
            target.push(item);
        }
    }
}

fn generate_code_files(
    handlebars: &Handlebars,
    language: &Language,
    output_path: &Path,
    data: &serde_json::Value,
) -> Result<()> {
    match language {
        Language::Rust => {
            let src_dir = output_path.join("src");
            fs::create_dir_all(&src_dir)?;

            let lib_content = handlebars.render("lib", data)?;
            fs::write(src_dir.join("lib.rs"), lib_content)?;

            let models_content = handlebars.render("models", data)?;
            fs::write(src_dir.join("models.rs"), models_content)?;
        }
        Language::Dart => {
            let lib_dir = output_path.join("lib");
            fs::create_dir_all(&lib_dir)?;

            let lib_content = handlebars.render("lib", data)?;
            fs::write(lib_dir.join("llm_models.dart"), lib_content)?;

            let models_content = handlebars.render("models", data)?;
            fs::write(lib_dir.join("models.dart"), models_content)?;
        }
        Language::Go => {
            let main_content = handlebars.render("main", data)?;
            fs::write(output_path.join("main.go"), main_content)?;

            let models_content = handlebars.render("models", data)?;
            fs::write(output_path.join("models.go"), models_content)?;
        }
        Language::Python => {
            let src_dir = output_path.join("src").join("llm_models");
            fs::create_dir_all(&src_dir)?;

            let init_content = handlebars.render("init", data)?;
            fs::write(src_dir.join("__init__.py"), init_content)?;

            let models_content = handlebars.render("models", data)?;
            fs::write(src_dir.join("models.py"), models_content)?;
        }
        Language::TypeScript => {
            let src_dir = output_path.join("src");
            fs::create_dir_all(&src_dir)?;

            let index_content = handlebars.render("index", data)?;
            fs::write(src_dir.join("index.ts"), index_content)?;

            let models_content = handlebars.render("models", data)?;
            fs::write(src_dir.join("models.ts"), models_content)?;

            let tsconfig_content = handlebars.render("tsconfig", data)?;
            fs::write(output_path.join("tsconfig.json"), tsconfig_content)?;
        }
    }

    Ok(())
}

fn generate_package_file(
    handlebars: &Handlebars,
    language: &Language,
    output_path: &Path,
    package_file: &str,
    data: &serde_json::Value,
) -> Result<()> {
    let template_name = match language {
        Language::Rust => "cargo",
        Language::Dart => "pubspec",
        Language::Go => "gomod",
        Language::Python => "pyproject",
        Language::TypeScript => "package",
    };

    let content = handlebars.render(template_name, data)?;
    fs::write(output_path.join(package_file), content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Cost, Limit, Modalities};

    fn sample_model() -> Model {
        Model {
            id: "gpt-test".to_string(),
            name: "GPT Test".to_string(),
            attachment: Some(true),
            reasoning: Some(true),
            temperature: Some(false),
            tool_call: Some(true),
            knowledge: Some("2024-01".to_string()),
            release_date: Some("2024-02-03".to_string()),
            last_updated: Some("2024-02-04".to_string()),
            modalities: Some(Modalities {
                input: Some(vec!["text".to_string(), "image".to_string()]),
                output: Some(vec!["text".to_string()]),
            }),
            open_weights: Some(false),
            cost: Some(Cost {
                input: Some(1.0),
                output: Some(2.0),
                cache_read: None,
                cache_write: None,
            }),
            limit: Some(Limit {
                context: Some(128000),
                output: Some(4096),
            }),
            provider: Provider {
                id: "openai".to_string(),
                name: "OpenAI".to_string(),
                description: None,
                website: Some("https://api.openai.com".to_string()),
            },
        }
    }

    #[test]
    fn normalize_models_flattens_fields_for_templates() {
        let models = vec![sample_model()];
        let normalized = normalize_models(&models);
        let model = &normalized[0];

        assert_eq!(model.context_length, Some(128000));
        assert_eq!(model.output_length, Some(4096));
        assert_eq!(model.input_cost, Some(1.0));
        assert_eq!(model.output_cost, Some(2.0));
        assert_eq!(model.function_calling, Some(true));
        assert_eq!(model.open_weight, Some(false));
        assert_eq!(model.knowledge_cutoff, Some("2024-01"));
        assert_eq!(model.modalities, vec!["text", "image"]);
        assert_eq!(model.provider.name, "OpenAI");
    }
}
