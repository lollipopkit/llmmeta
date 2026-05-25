use anyhow::{Context, Result};
use reqwest::Client;
use std::fs;

use crate::models::{Model, ModelsResponse, Provider};

const MODELS_API_URL: &str = "https://models.dev/api.json";

/// 从 models.dev API 获取模型数据
pub async fn fetch_models() -> Result<Vec<Model>> {
    let client = Client::new();

    let response = client
        .get(MODELS_API_URL)
        .header("User-Agent", "llmmeta/0.1.0")
        .send()
        .await
        .context("Failed to fetch models from API")?;

    let models_response: ModelsResponse = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    // Convert the provider-organized data into a flat list of models
    let mut all_models = Vec::new();
    for (provider_key, provider_data) in models_response {
        let (provider_id, provider_name) = provider_identity(&provider_key, &provider_data);
        let provider = Provider {
            id: provider_id,
            name: provider_name,
            description: None,
            website: provider_data.api.clone(),
        };

        for (model_id, mut model) in provider_data.models {
            // Ensure the model ID is set
            model.id = model_id;

            // Create provider info from the provider data
            model.provider = provider.clone();

            all_models.push(model);
        }
    }

    Ok(all_models)
}

fn provider_identity<'a>(
    provider_key: &'a str,
    provider_data: &'a crate::models::ProviderData,
) -> (String, String) {
    let provider_id = provider_data
        .id
        .as_deref()
        .filter(|id| !id.is_empty())
        .unwrap_or(provider_key);
    let provider_name = provider_data
        .name
        .as_deref()
        .filter(|name| !name.is_empty())
        .unwrap_or(provider_key);

    (provider_id.to_string(), provider_name.to_string())
}

/// 将模型数据保存到文件
pub fn save_models(models: &[Model], output_path: &str) -> Result<()> {
    let json =
        serde_json::to_string_pretty(models).context("Failed to serialize models to JSON")?;

    fs::write(output_path, json).context("Failed to write models to file")?;

    Ok(())
}

/// 从文件加载模型数据
pub fn load_models(input_path: &str) -> Result<Vec<Model>> {
    let content = fs::read_to_string(input_path).context("Failed to read models file")?;

    let models: Vec<Model> =
        serde_json::from_str(&content).context("Failed to parse models JSON")?;

    Ok(models)
}

/// 按提供商分组模型
pub fn group_models_by_provider(
    models: &[Model],
) -> std::collections::HashMap<String, Vec<&Model>> {
    let mut grouped = std::collections::HashMap::new();

    for model in models {
        grouped
            .entry(model.provider.name.clone())
            .or_insert_with(Vec::new)
            .push(model);
    }

    grouped
}

/// 筛选支持特定模态的模型
pub fn filter_by_modality<'a>(models: &'a [Model], modality: &str) -> Vec<&'a Model> {
    models
        .iter()
        .filter(|model| model.supports_modality(modality))
        .collect()
}

/// 筛选支持函数调用的模型
pub fn filter_function_calling_models(models: &[Model]) -> Vec<&Model> {
    models
        .iter()
        .filter(|model| model.supports_function_calling())
        .collect()
}

/// 按价格排序模型（从低到高）
pub fn sort_by_price(models: &[Model]) -> Vec<&Model> {
    let mut sorted: Vec<&Model> = models.iter().collect();

    sorted.sort_by(|a, b| match (a.get_pricing(), b.get_pricing()) {
        (Some((a_in, a_out)), Some((b_in, b_out))) => {
            let a_total = a_in + a_out;
            let b_total = b_in + b_out;
            a_total
                .partial_cmp(&b_total)
                .unwrap_or(std::cmp::Ordering::Equal)
        }
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });

    sorted
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
    fn saved_models_keep_provider_when_loaded() {
        let path = std::env::temp_dir().join(format!(
            "llmmeta-provider-roundtrip-{}.json",
            std::process::id()
        ));
        let path_string = path.to_string_lossy().into_owned();

        save_models(&[sample_model()], &path_string).unwrap();
        let loaded = load_models(&path_string).unwrap();
        let _ = std::fs::remove_file(path);

        assert_eq!(loaded[0].provider.id, "openai");
        assert_eq!(loaded[0].provider.name, "OpenAI");
        assert_eq!(
            loaded[0].provider.website.as_deref(),
            Some("https://api.openai.com")
        );
    }

    #[test]
    fn provider_data_can_fall_back_to_map_key_when_id_is_missing() {
        let json = r#"{
            "custom-provider": {
                "name": "Custom Provider",
                "api": "https://example.com/v1",
                "models": {
                    "custom-model": {
                        "name": "Custom Model"
                    }
                }
            }
        }"#;

        let parsed: crate::models::ModelsResponse = serde_json::from_str(json).unwrap();
        let provider = parsed.get("custom-provider").unwrap();

        assert_eq!(provider.id, None);
        assert_eq!(provider.name.as_deref(), Some("Custom Provider"));
        assert_eq!(provider.models["custom-model"].id, "");
    }

    #[test]
    fn provider_name_can_fall_back_to_map_key_when_name_is_missing() {
        let json = r#"{
            "custom-provider": {
                "id": "custom-provider",
                "api": "https://example.com/v1",
                "models": {
                    "custom-model": {
                        "name": "Custom Model"
                    }
                }
            }
        }"#;

        let parsed: crate::models::ModelsResponse = serde_json::from_str(json).unwrap();
        let provider = parsed.get("custom-provider").unwrap();
        let (provider_id, provider_name) = provider_identity("custom-provider", provider);

        assert_eq!(provider.id.as_deref(), Some("custom-provider"));
        assert_eq!(provider.name, None);
        assert_eq!(provider_id, "custom-provider");
        assert_eq!(provider_name, "custom-provider");
    }
}
