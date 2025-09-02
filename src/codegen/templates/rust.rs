pub const MODELS_TEMPLATE: &str = r#"//! LLM Models from models.dev
//! Generated at: {{timestamp}}
//! Total models: {{model_count}}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub context_length: Option<u32>,
    pub output_length: Option<u32>,
    pub input_cost: Option<f64>,
    pub output_cost: Option<f64>,
    pub release_date: Option<DateTime<Utc>>,
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
        self.modalities.contains(&modality.to_string())
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

/// All available models
pub const MODELS: &[Model] = &[
{{#each models}}
    Model {
        id: "{{id}}".to_string(),
        name: "{{name}}".to_string(),
        description: {{#if description}}Some("{{description}}".to_string()){{else}}None{{/if}},
        context_length: {{#if context_length}}Some({{context_length}}){{else}}None{{/if}},
        output_length: {{#if output_length}}Some({{output_length}}){{else}}None{{/if}},
        input_cost: {{#if input_cost}}Some({{input_cost}}){{else}}None{{/if}},
        output_cost: {{#if output_cost}}Some({{output_cost}}){{else}}None{{/if}},
        release_date: {{#if release_date}}Some("{{release_date}}".parse().unwrap()){{else}}None{{/if}},
        knowledge_cutoff: {{#if knowledge_cutoff}}Some("{{knowledge_cutoff}}".to_string()){{else}}None{{/if}},
        modalities: vec![{{~#if modalities~}}{{~#if modalities.input~}}{{#each modalities.input}}"{{this}}"{{#unless @last}}, {{/unless}}{{/each}}{{~/if~}}{{~#if modalities.output~}}{{#unless modalities.input}}{{/unless}}{{#if modalities.input}}, {{/if}}{{#each modalities.output}}"{{this}}"{{#unless @last}}, {{/unless}}{{/each}}{{~/if~}}{{~/if~}}].into_iter().map(String::from).collect(),
        reasoning: {{#if reasoning}}Some({{reasoning}}){{else}}None{{/if}},
        function_calling: {{#if function_calling}}Some({{function_calling}}){{else}}None{{/if}},
        tool_use: {{#if tool_use}}Some({{tool_use}}){{else}}None{{/if}},
        open_weight: {{#if open_weight}}Some({{open_weight}}){{else}}None{{/if}},
        provider: Provider {
            id: "{{provider.id}}".to_string(),
            name: "{{provider.name}}".to_string(),
            description: {{#if provider.description}}Some("{{provider.description}}".to_string()){{else}}None{{/if}},
            website: {{#if provider.website}}Some("{{provider.website}}".to_string()){{else}}None{{/if}},
        },
    },
{{/each}}
];
"#;

pub const LIB_TEMPLATE: &str = r#"//! LLM Models SDK for Rust
//! Generated from models.dev at: {{timestamp}}

pub mod models;

pub use models::{Model, Provider, MODELS};

/// Get all models
pub fn get_all_models() -> &'static [Model] {
    MODELS
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
name = "llm-models"
version = "0.1.0"
edition = "2021"
description = "LLM models data from models.dev"
license = "MIT"
repository = "https://github.com/your-username/llm-models-rust"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
"#;