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
    for (_provider_key, provider_data) in models_response {
        for (model_id, mut model) in provider_data.models {
            // Ensure the model ID is set
            model.id = model_id;
            
            // Create provider info from the provider data
            model.provider = Provider {
                id: provider_data.id.clone(),
                name: provider_data.name.clone(),
                description: None,
                website: provider_data.api.clone(),
            };
            
            all_models.push(model);
        }
    }

    Ok(all_models)
}

/// 将模型数据保存到文件
pub fn save_models(models: &[Model], output_path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(models)
        .context("Failed to serialize models to JSON")?;
    
    fs::write(output_path, json)
        .context("Failed to write models to file")?;
    
    Ok(())
}

/// 从文件加载模型数据
pub fn load_models(input_path: &str) -> Result<Vec<Model>> {
    let content = fs::read_to_string(input_path)
        .context("Failed to read models file")?;
    
    let models: Vec<Model> = serde_json::from_str(&content)
        .context("Failed to parse models JSON")?;
    
    Ok(models)
}

/// 按提供商分组模型
pub fn group_models_by_provider(models: &[Model]) -> std::collections::HashMap<String, Vec<&Model>> {
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
    
    sorted.sort_by(|a, b| {
        match (a.get_pricing(), b.get_pricing()) {
            (Some((a_in, a_out)), Some((b_in, b_out))) => {
                let a_total = a_in + a_out;
                let b_total = b_in + b_out;
                a_total.partial_cmp(&b_total).unwrap_or(std::cmp::Ordering::Equal)
            }
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
    
    sorted
}