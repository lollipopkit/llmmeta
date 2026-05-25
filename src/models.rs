use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn default_provider() -> Provider {
    Provider {
        id: "unknown".to_string(),
        name: "Unknown".to_string(),
        description: None,
        website: None,
    }
}

// The response from models.dev is a map of provider names to provider info
pub type ModelsResponse = HashMap<String, ProviderData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderData {
    pub id: Option<String>,
    pub name: Option<String>,
    pub api: Option<String>,
    pub doc: Option<String>,
    pub npm: Option<String>,
    pub env: Option<Vec<String>>,
    pub models: HashMap<String, Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub attachment: Option<bool>,
    pub reasoning: Option<bool>,
    pub temperature: Option<bool>,
    pub tool_call: Option<bool>,
    pub knowledge: Option<String>,
    pub release_date: Option<String>,
    pub last_updated: Option<String>,
    pub modalities: Option<Modalities>,
    pub open_weights: Option<bool>,
    pub cost: Option<Cost>,
    pub limit: Option<Limit>,
    #[serde(default = "default_provider")]
    pub provider: Provider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modalities {
    pub input: Option<Vec<String>>,
    pub output: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cost {
    pub input: Option<f64>,
    pub output: Option<f64>,
    pub cache_read: Option<f64>,
    pub cache_write: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    pub context: Option<u32>,
    pub output: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
}

impl Model {
    /// 检查模型是否支持指定的模态
    pub fn supports_modality(&self, modality: &str) -> bool {
        if let Some(ref modalities) = self.modalities {
            if let Some(ref input) = modalities.input {
                if input.contains(&modality.to_string()) {
                    return true;
                }
            }
            if let Some(ref output) = modalities.output {
                if output.contains(&modality.to_string()) {
                    return true;
                }
            }
        }
        false
    }

    /// 获取模型的定价信息
    pub fn get_pricing(&self) -> Option<(f64, f64)> {
        if let Some(ref cost) = self.cost {
            match (cost.input, cost.output) {
                (Some(input), Some(output)) => Some((input, output)),
                _ => None,
            }
        } else {
            None
        }
    }

    /// 检查模型是否支持函数调用
    pub fn supports_function_calling(&self) -> bool {
        self.tool_call.unwrap_or(false)
    }

    /// 检查模型是否具有推理能力
    pub fn has_reasoning(&self) -> bool {
        self.reasoning.unwrap_or(false)
    }

    /// 检查模型是否为开源模型
    pub fn is_open_source(&self) -> bool {
        self.open_weights.unwrap_or(false)
    }

    /// 获取上下文长度
    pub fn get_context_length(&self) -> Option<u32> {
        self.limit.as_ref().and_then(|l| l.context)
    }

    /// 获取输出长度限制
    pub fn get_output_length(&self) -> Option<u32> {
        self.limit.as_ref().and_then(|l| l.output)
    }
}
