use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::models::Model;

mod api;
mod codegen;
mod models;

#[derive(Parser)]
#[command(name = "llmmeta")]
#[command(about = "自动抓取 models.dev API 数据并生成多语言 SDK")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 从 models.dev 获取最新模型数据
    Fetch {
        /// 输出文件路径
        #[arg(short, long, default_value = "models.json")]
        output: String,
    },
    /// 生成指定语言的 SDK 代码
    Generate {
        /// 模型数据文件路径
        #[arg(short, long, default_value = "models.json")]
        input: String,
        /// 目标语言 (rust, dart, golang, python, typescript)
        #[arg(short, long)]
        lang: String,
        /// 输出目录
        #[arg(short, long, default_value = "output")]
        output: String,
    },
    /// 分析和筛选模型数据
    Analyze {
        /// 模型数据文件路径
        #[arg(short, long, default_value = "models.json")]
        input: String,
        /// 按模态筛选 (如: text, image, audio)
        #[arg(long)]
        modality: Option<String>,
        /// 只显示支持函数调用的模型
        #[arg(long)]
        function_calling: bool,
        /// 只显示支持推理的模型
        #[arg(long)]
        reasoning: bool,
        /// 只显示开源模型
        #[arg(long)]
        open_source: bool,
        /// 按价格排序
        #[arg(long)]
        sort_by_price: bool,
        /// 按提供商分组显示
        #[arg(long)]
        group_by_provider: bool,
        /// 输出格式 (table, json)
        #[arg(long, default_value = "table")]
        format: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Fetch { output } => {
            println!("正在从 models.dev 获取模型数据...");
            let models = api::fetch_models().await?;
            api::save_models(&models, output)?;
            println!("模型数据已保存到: {}", output);
        }
        Commands::Generate {
            input,
            lang,
            output,
        } => {
            println!("正在生成 {} 语言的 SDK...", lang);
            let models = api::load_models(input)?;
            codegen::generate_sdk(&models, lang, output)?;
            println!("SDK 已生成到: {}", output);
        }
        Commands::Analyze {
            input,
            modality,
            function_calling,
            reasoning,
            open_source,
            sort_by_price,
            group_by_provider,
            format,
        } => {
            let models = api::load_models(input)?;

            // 应用筛选条件
            let mut filtered_models = models.clone();

            // 使用专用的筛选函数
            if let Some(ref modal) = modality {
                let filtered_refs = api::filter_by_modality(&filtered_models, modal);
                filtered_models = filtered_refs.into_iter().cloned().collect();
            }

            if *function_calling {
                let filtered_refs = api::filter_function_calling_models(&filtered_models);
                filtered_models = filtered_refs.into_iter().cloned().collect();
            }

            // 应用其他筛选条件
            filtered_models.retain(|m| {
                if *reasoning && !m.has_reasoning() {
                    return false;
                }
                if *open_source && !m.is_open_source() {
                    return false;
                }
                true
            });

            // 应用排序
            let result_models: Vec<&Model> = if *sort_by_price {
                api::sort_by_price(&filtered_models)
            } else {
                filtered_models.iter().collect()
            };

            // 显示结果
            if *group_by_provider {
                display_grouped_models(&result_models, format)?;
            } else {
                display_models(&result_models, format)?;
            }
        }
    }

    Ok(())
}

fn display_models(models: &[&Model], format: &str) -> Result<()> {
    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(models)?);
        }
        "table" => {
            println!(
                "{:<20} {:<30} {:<15} {:<10} {:<10} {:<15} {:<20} {:<15}",
                "Provider",
                "Model",
                "Modalities",
                "Function",
                "Reasoning",
                "Open Source",
                "Pricing",
                "Context/Output"
            );
            println!("{}", "=".repeat(155));

            for model in models {
                let modalities = if let Some(ref mod_data) = model.modalities {
                    let mut inputs = mod_data
                        .input
                        .as_ref()
                        .map(|v| v.join(","))
                        .unwrap_or_default();
                    if inputs.len() > 10 {
                        inputs.truncate(10);
                        inputs.push_str("...");
                    }
                    inputs
                } else {
                    "N/A".to_string()
                };

                let pricing = if let Some((input, output)) = model.get_pricing() {
                    format!("${:.4}/{:.4}", input, output)
                } else {
                    "N/A".to_string()
                };

                let limits = match (model.get_context_length(), model.get_output_length()) {
                    (Some(ctx), Some(out)) => format!("{}/{}", ctx, out),
                    (Some(ctx), None) => format!("{}/N/A", ctx),
                    (None, Some(out)) => format!("N/A/{}", out),
                    (None, None) => "N/A/N/A".to_string(),
                };

                println!(
                    "{:<20} {:<30} {:<15} {:<10} {:<10} {:<15} {:<20} {:<15}",
                    model.provider.name,
                    if model.name.len() > 28 {
                        format!("{}...", &model.name[..25])
                    } else {
                        model.name.clone()
                    },
                    modalities,
                    if model.supports_function_calling() {
                        "Yes"
                    } else {
                        "No"
                    },
                    if model.has_reasoning() { "Yes" } else { "No" },
                    if model.is_open_source() { "Yes" } else { "No" },
                    pricing,
                    limits
                );
            }

            println!("\nTotal models: {}", models.len());
        }
        _ => {
            eprintln!("Unsupported format: {}. Use 'table' or 'json'.", format);
        }
    }
    Ok(())
}

fn display_grouped_models(models: &[&Model], format: &str) -> Result<()> {
    match format {
        "json" => {
            let grouped = group_model_refs_by_provider(models);
            println!("{}", serde_json::to_string_pretty(&grouped)?);
        }
        "table" => {
            let grouped = group_model_refs_by_provider(models);

            for (provider, provider_models) in grouped {
                println!("\n=== {} ({} models) ===", provider, provider_models.len());
                display_models(&provider_models, "table")?;
            }
        }
        _ => {
            eprintln!("Unsupported format: {}. Use 'table' or 'json'.", format);
        }
    }
    Ok(())
}

fn group_model_refs_by_provider<'a>(
    models: &[&'a Model],
) -> std::collections::HashMap<String, Vec<&'a Model>> {
    let mut grouped = std::collections::HashMap::new();

    for model in models {
        grouped
            .entry(model.provider.name.clone())
            .or_insert_with(Vec::new)
            .push(*model);
    }

    grouped
}
