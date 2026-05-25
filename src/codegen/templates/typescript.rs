pub const MODELS_TEMPLATE: &str = r#"/**
 * LLM Models from models.dev
 * Generated at: {{timestamp}}
 * Total models: {{model_count}}
 */

export interface Provider {
  id: string;
  name: string;
  description?: string | null;
  website?: string | null;
}

export interface Model {
  id: string;
  name: string;
  description?: string | null;
  contextLength?: number | null;
  outputLength?: number | null;
  inputCost?: number | null;
  outputCost?: number | null;
  releaseDate?: string | null;
  knowledgeCutoff?: string | null;
  modalities: string[];
  reasoning?: boolean | null;
  functionCalling?: boolean | null;
  toolUse?: boolean | null;
  openWeight?: boolean | null;
  provider: Provider;
}

interface RawModel {
  id: string;
  name: string;
  description: string | null;
  context_length: number | null;
  output_length: number | null;
  input_cost: number | null;
  output_cost: number | null;
  release_date: string | null;
  knowledge_cutoff: string | null;
  modalities: string[];
  reasoning: boolean | null;
  function_calling: boolean | null;
  tool_use: boolean | null;
  open_weight: boolean | null;
  provider: Provider;
}

export class ModelHelper {
  constructor(private model: Model) {}

  supportsModality(modality: string): boolean {
    return this.model.modalities.includes(modality);
  }

  getPricing(): [number, number] | null {
    if (this.model.inputCost !== undefined && this.model.inputCost !== null &&
        this.model.outputCost !== undefined && this.model.outputCost !== null) {
      return [this.model.inputCost, this.model.outputCost];
    }
    return null;
  }

  supportsFunctionCalling(): boolean {
    return Boolean(this.model.functionCalling) || Boolean(this.model.toolUse);
  }

  hasReasoning(): boolean {
    return Boolean(this.model.reasoning);
  }

  isOpenSource(): boolean {
    return Boolean(this.model.openWeight);
  }
}

const modelsJson = {{{models_json_literal}}};
const rawModels = JSON.parse(modelsJson) as RawModel[];

// All available models
export const models: Model[] = rawModels.map((model) => ({
  id: model.id,
  name: model.name,
  description: model.description,
  contextLength: model.context_length,
  outputLength: model.output_length,
  inputCost: model.input_cost,
  outputCost: model.output_cost,
  releaseDate: model.release_date,
  knowledgeCutoff: model.knowledge_cutoff,
  modalities: model.modalities,
  reasoning: model.reasoning,
  functionCalling: model.function_calling,
  toolUse: model.tool_use,
  openWeight: model.open_weight,
  provider: model.provider,
}));

// Helper function to create ModelHelper instances
export const createModelHelper = (model: Model): ModelHelper => new ModelHelper(model);
"#;

pub const INDEX_TEMPLATE: &str = r#"/**
 * LLM Models SDK for TypeScript/JavaScript
 * Generated from models.dev at: {{timestamp}}
 */

import { Model, Provider, models, ModelHelper, createModelHelper } from './models';

export { Model, Provider, ModelHelper };

/**
 * Get all available models
 */
export function getAllModels(): Model[] {
  return models;
}

/**
 * Get models from a specific provider
 */
export function getModelsByProvider(providerName: string): Model[] {
  return models.filter(model => model.provider.name === providerName);
}

/**
 * Get models that support a specific modality
 */
export function getModelsByModality(modality: string): Model[] {
  return models.filter(model => createModelHelper(model).supportsModality(modality));
}

/**
 * Get models that support function calling
 */
export function getFunctionCallingModels(): Model[] {
  return models.filter(model => createModelHelper(model).supportsFunctionCalling());
}

/**
 * Get models with reasoning capabilities
 */
export function getReasoningModels(): Model[] {
  return models.filter(model => createModelHelper(model).hasReasoning());
}

/**
 * Get open source models
 */
export function getOpenSourceModels(): Model[] {
  return models.filter(model => createModelHelper(model).isOpenSource());
}

/**
 * Get models sorted by total price (low to high)
 */
export function getModelsSortedByPrice(): Model[] {
  const modelsWithPricing = models.filter(model => {
    const helper = createModelHelper(model);
    return helper.getPricing() !== null;
  });
  
  return modelsWithPricing.sort((a, b) => {
    const pricingA = createModelHelper(a).getPricing()!;
    const pricingB = createModelHelper(b).getPricing()!;
    const totalA = pricingA[0] + pricingA[1];
    const totalB = pricingB[0] + pricingB[1];
    return totalA - totalB;
  });
}

/**
 * Search models by name or description
 */
export function searchModels(query: string): Model[] {
  const queryLower = query.toLowerCase();
  return models.filter(model => 
    model.name.toLowerCase().includes(queryLower) ||
    (model.description && model.description.toLowerCase().includes(queryLower))
  );
}

/**
 * Get all unique provider names
 */
export function getProviders(): string[] {
  const providerNames = new Set(models.map(model => model.provider.name));
  return Array.from(providerNames).sort();
}

/**
 * Get a specific model by ID
 */
export function getModelById(modelId: string): Model | undefined {
  return models.find(model => model.id === modelId);
}

/**
 * Get models by context length range
 */
export function getModelsByContextLength(minLength?: number, maxLength?: number): Model[] {
  return models.filter(model => {
    if (model.contextLength === undefined || model.contextLength === null) return false;
    if (minLength !== undefined && model.contextLength < minLength) return false;
    if (maxLength !== undefined && model.contextLength > maxLength) return false;
    return true;
  });
}

/**
 * Get the cheapest model for input/output
 */
export function getCheapestModel(): Model | undefined {
  const sortedByPrice = getModelsSortedByPrice();
  return sortedByPrice.length > 0 ? sortedByPrice[0] : undefined;
}

/**
 * Get models with the largest context window
 */
export function getLargestContextModels(limit: number = 10): Model[] {
  return models
    .filter(model => model.contextLength !== undefined && model.contextLength !== null)
    .sort((a, b) => (b.contextLength || 0) - (a.contextLength || 0))
    .slice(0, limit);
}
"#;

pub const PACKAGE_TEMPLATE: &str = r#"{
  "name": "llm-models",
  "version": "0.1.0",
  "description": "LLM models data from models.dev",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "files": [
    "dist/**/*"
  ],
  "scripts": {
    "build": "tsc",
    "prepublishOnly": "npm run build"
  },
  "keywords": [
    "llm",
    "ai",
    "models",
    "openai",
    "anthropic",
    "google",
    "meta"
  ],
  "author": "LLM Meta",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/your-username/llm-models-typescript.git"
  },
  "bugs": {
    "url": "https://github.com/your-username/llm-models-typescript/issues"
  },
  "homepage": "https://github.com/your-username/llm-models-typescript#readme",
  "devDependencies": {
    "typescript": "^5.0.0"
  }
}
"#;

pub const TSCONFIG_TEMPLATE: &str = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "CommonJS",
    "moduleResolution": "node",
    "declaration": true,
    "outDir": "dist",
    "rootDir": "src",
    "strict": true,
    "skipLibCheck": true
  },
  "include": [
    "src/**/*.ts"
  ]
}
"#;
