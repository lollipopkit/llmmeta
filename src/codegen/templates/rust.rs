pub const MODELS_TEMPLATE: &str = r#"//! LLM Models from models.dev
//! Generated at: {{timestamp}}
//! Total models: {{model_count}}

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub context_length: Option<u32>,
    pub output_length: Option<u32>,
    pub input_cost: Option<f64>,
    pub output_cost: Option<f64>,
    pub release_date: Option<String>,
    pub knowledge_cutoff: Option<String>,
    pub modalities: Vec<String>,
    pub reasoning: Option<bool>,
    pub function_calling: Option<bool>,
    pub tool_use: Option<bool>,
    pub open_weight: Option<bool>,
    pub provider: Provider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
}

impl Model {
    pub fn supports_modality(&self, modality: &str) -> bool {
        self.modalities.iter().any(|item| item == modality)
    }

    pub fn get_pricing(&self) -> Option<(f64, f64)> {
        match (self.input_cost, self.output_cost) {
            (Some(input), Some(output)) => Some((input, output)),
            _ => None,
        }
    }

    pub fn supports_function_calling(&self) -> bool {
        self.function_calling.unwrap_or(false) || self.tool_use.unwrap_or(false)
    }

    pub fn has_reasoning(&self) -> bool {
        self.reasoning.unwrap_or(false)
    }

    pub fn is_open_source(&self) -> bool {
        self.open_weight.unwrap_or(false)
    }
}

const MODELS_JSON: &str = {{{models_json_literal}}};

/// All available models.
pub static MODELS: LazyLock<Vec<Model>> = LazyLock::new(|| {
    serde_json::from_str(MODELS_JSON).expect("generated models JSON should be valid")
});

#[cfg(test)]
mod tests {
    use super::MODELS;

    #[test]
    fn generated_models_json_loads() {
        assert!(!MODELS.is_empty());
    }
}
"#;

pub const LIB_TEMPLATE: &str = r#"//! LLM Models SDK for Rust
//! Generated from models.dev at: {{timestamp}}

pub mod models;

pub use models::{Model, Provider, MODELS};

/// Get all models
pub fn get_all_models() -> &'static [Model] {
    &MODELS
}

/// Get models by provider name
pub fn get_models_by_provider(provider_name: &str) -> Vec<&'static Model> {
    MODELS
        .iter()
        .filter(|model| model.provider.name == provider_name)
        .collect()
}

/// Get models that support a specific modality
pub fn get_models_by_modality(modality: &str) -> Vec<&'static Model> {
    MODELS
        .iter()
        .filter(|model| model.supports_modality(modality))
        .collect()
}

/// Get models with function calling support
pub fn get_function_calling_models() -> Vec<&'static Model> {
    MODELS
        .iter()
        .filter(|model| model.supports_function_calling())
        .collect()
}

/// Get models with reasoning capabilities
pub fn get_reasoning_models() -> Vec<&'static Model> {
    MODELS
        .iter()
        .filter(|model| model.has_reasoning())
        .collect()
}

/// Get open source models
pub fn get_open_source_models() -> Vec<&'static Model> {
    MODELS
        .iter()
        .filter(|model| model.is_open_source())
        .collect()
}
"#;

pub const CARGO_TEMPLATE: &str = r#"[package]
name = "{{package_name}}"
version = "{{package_version}}"
edition = "2021"
description = "LLM models data from models.dev"
license = "MIT"
repository = "{{repository}}"
homepage = "{{package_homepage}}"
documentation = "https://docs.rs/{{package_name}}"
readme = "README.md"
keywords = ["llm", "ai", "models", "metadata"]
categories = ["api-bindings", "data-structures"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
"#;
