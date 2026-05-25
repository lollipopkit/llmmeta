use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::Path;

use crate::models::{Model, Provider};

mod templates;

pub const DEFAULT_PACKAGE_VERSION: &str = "0.1.0";
pub const DEFAULT_REPOSITORY: &str = "https://github.com/lollipopkit/llmmeta";
pub const DEFAULT_GO_MODULE: &str = "github.com/lollipopkit/llmmeta/sdks/go";
pub const DEFAULT_RUST_PACKAGE_NAME: &str = "llm-meta";
pub const DEFAULT_DART_PACKAGE_NAME: &str = "llm_meta";
pub const DEFAULT_GO_PACKAGE_NAME: &str = "llmmeta";
pub const DEFAULT_PYTHON_PACKAGE_NAME: &str = "llm-meta";
pub const DEFAULT_TYPESCRIPT_PACKAGE_NAME: &str = "llm-meta";
pub const DART_LIBRARY_FILE_NAME: &str = "llm_meta.dart";
pub const DART_LIBRARY_NAME: &str = "llm_meta";
pub const PYTHON_IMPORT_PACKAGE_NAME: &str = "llm_meta";

/// SDK generation options that affect package publishing metadata.
#[derive(Debug, Clone)]
pub struct GenerateOptions {
    pub package_version: String,
    pub repository: String,
    pub package_name: Option<String>,
    pub go_module: String,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            package_version: DEFAULT_PACKAGE_VERSION.to_string(),
            repository: DEFAULT_REPOSITORY.to_string(),
            package_name: None,
            go_module: DEFAULT_GO_MODULE.to_string(),
        }
    }
}

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

    pub fn default_package_name(&self) -> &'static str {
        match self {
            Language::Rust => DEFAULT_RUST_PACKAGE_NAME,
            Language::Dart => DEFAULT_DART_PACKAGE_NAME,
            Language::Go => DEFAULT_GO_PACKAGE_NAME,
            Language::Python => DEFAULT_PYTHON_PACKAGE_NAME,
            Language::TypeScript => DEFAULT_TYPESCRIPT_PACKAGE_NAME,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::Dart => "Dart",
            Language::Go => "Go",
            Language::Python => "Python",
            Language::TypeScript => "TypeScript",
        }
    }
}

/// 生成指定语言的 SDK
pub fn generate_sdk(
    models: &[Model],
    lang: &str,
    output_dir: &str,
    options: &GenerateOptions,
) -> Result<()> {
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
    let package_name = match options.package_name.as_deref().map(str::trim) {
        Some("") => {
            eprintln!(
                "package name is empty; using default `{}`",
                language.default_package_name()
            );
            language.default_package_name()
        }
        Some(value) => value,
        None => language.default_package_name(),
    };
    let repository = trimmed_or_default(&options.repository, DEFAULT_REPOSITORY, "repository")
        .trim_end_matches('/');
    let go_module_path = trimmed_or_default(&options.go_module, DEFAULT_GO_MODULE, "go module")
        .trim_end_matches('/');
    let package_version = trimmed_or_default(
        &options.package_version,
        DEFAULT_PACKAGE_VERSION,
        "package version",
    );
    let go_package_name = sanitize_go_package_name(package_name);
    let repository_directory = sdk_repository_directory(&language);
    let package_homepage = format!("{repository}/tree/main/{repository_directory}");
    let package_issues = format!("{repository}/issues");

    // 准备模板数据
    let template_data = json!({
        "models": codegen_models,
        "models_json_literal": models_json_literal,
        "models_json_literal_dart": models_json_literal_dart,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "model_count": models.len(),
        "providers": crate::api::group_models_by_provider(models).keys().collect::<Vec<_>>(),
        "package_name": package_name,
        "package_version": package_version,
        "repository": repository,
        "repository_directory": repository_directory,
        "package_homepage": package_homepage,
        "package_issues": package_issues,
        "go_module_path": go_module_path,
        "go_package_name": go_package_name,
        "dart_library_name": DART_LIBRARY_NAME,
        "python_import_package_name": PYTHON_IMPORT_PACKAGE_NAME,
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

    generate_support_files(
        &language,
        output_path,
        package_name,
        package_version,
        go_module_path,
    )?;

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
            fs::write(lib_dir.join(DART_LIBRARY_FILE_NAME), lib_content)?;

            let models_content = handlebars.render("models", data)?;
            fs::write(lib_dir.join("models.dart"), models_content)?;
        }
        Language::Go => {
            let main_content = handlebars.render("main", data)?;
            fs::write(output_path.join("llmmeta.go"), main_content)?;

            let models_content = handlebars.render("models", data)?;
            fs::write(output_path.join("models.go"), models_content)?;
        }
        Language::Python => {
            let src_dir = output_path.join("src").join(PYTHON_IMPORT_PACKAGE_NAME);
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

fn generate_support_files(
    language: &Language,
    output_path: &Path,
    package_name: &str,
    package_version: &str,
    go_module_path: &str,
) -> Result<()> {
    fs::write(
        output_path.join("README.md"),
        package_readme(language, package_name, package_version, go_module_path),
    )?;
    fs::write(
        output_path.join("CHANGELOG.md"),
        format!("# Changelog\n\n## {package_version}\n\n- Generated models.dev SDK package.\n"),
    )?;
    fs::write(output_path.join("LICENSE"), mit_license())?;

    Ok(())
}

fn package_readme(
    language: &Language,
    package_name: &str,
    package_version: &str,
    go_module_path: &str,
) -> String {
    let install = match language {
        Language::Rust => {
            format!("```toml\n[dependencies]\n{package_name} = \"{package_version}\"\n```")
        }
        Language::Dart => {
            format!("```yaml\ndependencies:\n  {package_name}: ^{package_version}\n```")
        }
        Language::Go => format!("```sh\ngo get {go_module_path}\n```"),
        Language::Python => format!("```sh\npip install {package_name}\n```"),
        Language::TypeScript => format!("```sh\nnpm install {package_name}\n```"),
    };

    format!(
        "# {package_name}\n\nGenerated {language} SDK for models.dev LLM metadata.\n\n## Install\n\n{install}\n\n## Usage\n\nUse the exported model list and helper functions to query providers, modalities, pricing, reasoning support, function calling support, and open-weight status.\n",
        language = language.label()
    )
}

fn mit_license() -> &'static str {
    "MIT License\n\nCopyright (c) LLM Meta contributors\n\nPermission is hereby granted, free of charge, to any person obtaining a copy\nof this software and associated documentation files (the \"Software\"), to deal\nin the Software without restriction, including without limitation the rights\nto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\ncopies of the Software, and to permit persons to whom the Software is\nfurnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice shall be included in all\ncopies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\nFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\nAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\nLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\nOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\nSOFTWARE.\n"
}

fn sdk_repository_directory(language: &Language) -> &'static str {
    match language {
        Language::Rust => "sdks/rust",
        Language::Dart => "sdks/dart",
        Language::Go => "sdks/go",
        Language::Python => "sdks/python",
        Language::TypeScript => "sdks/typescript",
    }
}

fn trimmed_or_default<'a>(value: &'a str, default: &'static str, label: &str) -> &'a str {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        eprintln!("{label} is empty; using default `{default}`");
        default
    } else {
        trimmed
    }
}

fn sanitize_go_package_name(name: &str) -> String {
    let mut output = String::new();
    for character in name.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            output.push(character);
        }
    }

    if output.is_empty() || output.as_bytes()[0].is_ascii_digit() {
        output.insert_str(0, "llm");
    }

    output.to_ascii_lowercase()
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
    use std::time::{SystemTime, UNIX_EPOCH};

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

    #[test]
    fn generate_sdk_uses_default_for_blank_package_version() {
        let output_dir = std::env::temp_dir().join(format!(
            "llmmeta-blank-version-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos()
        ));
        let options = GenerateOptions {
            package_version: "   ".to_string(),
            ..GenerateOptions::default()
        };

        generate_sdk(
            &[sample_model()],
            "rust",
            output_dir.to_str().unwrap(),
            &options,
        )
        .expect("rust SDK should generate with default package version");

        let cargo_toml = fs::read_to_string(output_dir.join("Cargo.toml"))
            .expect("generated Cargo.toml should be readable");
        assert!(cargo_toml.contains(&format!("version = \"{DEFAULT_PACKAGE_VERSION}\"")));
        assert!(!cargo_toml.contains("version = \"   \""));

        fs::remove_dir_all(output_dir).expect("test output directory should be removable");
    }
}
