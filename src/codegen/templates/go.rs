pub const MODELS_TEMPLATE: &str = r#"// LLM Models from models.dev
// Generated at: {{timestamp}}
// Total models: {{model_count}}

package {{go_package_name}}

import "encoding/json"

type Model struct {
	ID              string   `json:"id"`
	Name            string   `json:"name"`
	Description     *string  `json:"description,omitempty"`
	ContextLength   *uint32  `json:"context_length,omitempty"`
	OutputLength    *uint32  `json:"output_length,omitempty"`
	InputCost       *float64 `json:"input_cost,omitempty"`
	OutputCost      *float64 `json:"output_cost,omitempty"`
	ReleaseDate     *string  `json:"release_date,omitempty"`
	KnowledgeCutoff *string  `json:"knowledge_cutoff,omitempty"`
	Modalities      []string `json:"modalities"`
	Reasoning       *bool    `json:"reasoning,omitempty"`
	FunctionCalling *bool    `json:"function_calling,omitempty"`
	ToolUse         *bool    `json:"tool_use,omitempty"`
	OpenWeight      *bool    `json:"open_weight,omitempty"`
	Provider        Provider `json:"provider"`
}

type Provider struct {
	ID          string  `json:"id"`
	Name        string  `json:"name"`
	Description *string `json:"description,omitempty"`
	Website     *string `json:"website,omitempty"`
}

func (m *Model) SupportsModality(modality string) bool {
	for _, mod := range m.Modalities {
		if mod == modality {
			return true
		}
	}
	return false
}

func (m *Model) GetPricing() (float64, float64, bool) {
	if m.InputCost != nil && m.OutputCost != nil {
		return *m.InputCost, *m.OutputCost, true
	}
	return 0, 0, false
}

func (m *Model) SupportsFunctionCalling() bool {
	return (m.FunctionCalling != nil && *m.FunctionCalling) ||
		(m.ToolUse != nil && *m.ToolUse)
}

func (m *Model) HasReasoning() bool {
	return m.Reasoning != nil && *m.Reasoning
}

func (m *Model) IsOpenSource() bool {
	return m.OpenWeight != nil && *m.OpenWeight
}

const modelsJSON = {{{models_json_literal}}}

// All available models
var Models []Model

func init() {
	if err := json.Unmarshal([]byte(modelsJSON), &Models); err != nil {
		panic(err)
	}
}
"#;

pub const MAIN_TEMPLATE: &str = r#"// LLM Models SDK for Go
// Generated from models.dev at: {{timestamp}}

package {{go_package_name}}

import "sort"

// GetAllModels returns all available models
func GetAllModels() []Model {
	return Models
}

// GetModelsByProvider returns models from a specific provider
func GetModelsByProvider(providerName string) []Model {
	var result []Model
	for _, model := range Models {
		if model.Provider.Name == providerName {
			result = append(result, model)
		}
	}
	return result
}

// GetModelsByModality returns models that support a specific modality
func GetModelsByModality(modality string) []Model {
	var result []Model
	for _, model := range Models {
		if model.SupportsModality(modality) {
			result = append(result, model)
		}
	}
	return result
}

// GetFunctionCallingModels returns models that support function calling
func GetFunctionCallingModels() []Model {
	var result []Model
	for _, model := range Models {
		if model.SupportsFunctionCalling() {
			result = append(result, model)
		}
	}
	return result
}

// GetReasoningModels returns models with reasoning capabilities
func GetReasoningModels() []Model {
	var result []Model
	for _, model := range Models {
		if model.HasReasoning() {
			result = append(result, model)
		}
	}
	return result
}

// GetOpenSourceModels returns open source models
func GetOpenSourceModels() []Model {
	var result []Model
	for _, model := range Models {
		if model.IsOpenSource() {
			result = append(result, model)
		}
	}
	return result
}

// GetModelsSortedByPrice returns models sorted by total price (low to high)
func GetModelsSortedByPrice() []Model {
	var modelsWithPricing []Model
	for _, model := range Models {
		if _, _, hasPricing := model.GetPricing(); hasPricing {
			modelsWithPricing = append(modelsWithPricing, model)
		}
	}

	sort.Slice(modelsWithPricing, func(i, j int) bool {
		inputA, outputA, _ := modelsWithPricing[i].GetPricing()
		inputB, outputB, _ := modelsWithPricing[j].GetPricing()
		totalA := inputA + outputA
		totalB := inputB + outputB
		return totalA < totalB
	})

	return modelsWithPricing
}
"#;

pub const GOMOD_TEMPLATE: &str = r#"module {{go_module_path}}

go 1.21
"#;
