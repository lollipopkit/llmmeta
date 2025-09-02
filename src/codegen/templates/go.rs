pub const MODELS_TEMPLATE: &str = r#"// LLM Models from models.dev
// Generated at: {{timestamp}}
// Total models: {{model_count}}

package main

import (
	"time"
)

type Model struct {
	ID               string    `json:"id"`
	Name             string    `json:"name"`
	Description      *string   `json:"description,omitempty"`
	ContextLength    *uint32   `json:"context_length,omitempty"`
	OutputLength     *uint32   `json:"output_length,omitempty"`
	InputCost        *float64  `json:"input_cost,omitempty"`
	OutputCost       *float64  `json:"output_cost,omitempty"`
	ReleaseDate      *time.Time `json:"release_date,omitempty"`
	KnowledgeCutoff  *string   `json:"knowledge_cutoff,omitempty"`
	Modalities       []string  `json:"modalities"`
	Reasoning        *bool     `json:"reasoning,omitempty"`
	FunctionCalling  *bool     `json:"function_calling,omitempty"`
	ToolUse          *bool     `json:"tool_use,omitempty"`
	OpenWeight       *bool     `json:"open_weight,omitempty"`
	Provider         Provider  `json:"provider"`
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

// Helper function for string pointers
func stringPtr(s string) *string {
	if s == "" {
		return nil
	}
	return &s
}

// Helper function for bool pointers
func boolPtr(b bool) *bool {
	return &b
}

// Helper function for uint32 pointers
func uint32Ptr(u uint32) *uint32 {
	if u == 0 {
		return nil
	}
	return &u
}

// Helper function for float64 pointers
func float64Ptr(f float64) *float64 {
	if f == 0 {
		return nil
	}
	return &f
}

// Helper function for time pointers
func timePtr(t string) *time.Time {
	if t == "" {
		return nil
	}
	parsed, err := time.Parse(time.RFC3339, t)
	if err != nil {
		return nil
	}
	return &parsed
}

// All available models
var Models = []Model{
{{#each models}}
	{
		ID:               "{{id}}",
		Name:             "{{name}}",
		Description:      {{#if description}}stringPtr("{{description}}"){{else}}nil{{/if}},
		ContextLength:    {{#if context_length}}uint32Ptr({{context_length}}){{else}}nil{{/if}},
		OutputLength:     {{#if output_length}}uint32Ptr({{output_length}}){{else}}nil{{/if}},
		InputCost:        {{#if input_cost}}float64Ptr({{input_cost}}){{else}}nil{{/if}},
		OutputCost:       {{#if output_cost}}float64Ptr({{output_cost}}){{else}}nil{{/if}},
		ReleaseDate:      {{#if release_date}}timePtr("{{release_date}}"){{else}}nil{{/if}},
		KnowledgeCutoff:  {{#if knowledge_cutoff}}stringPtr("{{knowledge_cutoff}}"){{else}}nil{{/if}},
		Modalities:       []string{ {{~#each modalities~}} "{{this}}"{{~#unless @last~}}, {{~/unless~}} {{~/each~}} },
		Reasoning:        {{#if reasoning}}boolPtr({{reasoning}}){{else}}nil{{/if}},
		FunctionCalling:  {{#if function_calling}}boolPtr({{function_calling}}){{else}}nil{{/if}},
		ToolUse:          {{#if tool_use}}boolPtr({{tool_use}}){{else}}nil{{/if}},
		OpenWeight:       {{#if open_weight}}boolPtr({{open_weight}}){{else}}nil{{/if}},
		Provider: Provider{
			ID:          "{{provider.id}}",
			Name:        "{{provider.name}}",
			Description: {{#if provider.description}}stringPtr("{{provider.description}}"){{else}}nil{{/if}},
			Website:     {{#if provider.website}}stringPtr("{{provider.website}}"){{else}}nil{{/if}},
		},
	},
{{/each}}
}
"#;

pub const MAIN_TEMPLATE: &str = r#"// LLM Models SDK for Go
// Generated from models.dev at: {{timestamp}}

package main

import (
	"encoding/json"
	"fmt"
	"sort"
)

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

func main() {
	fmt.Printf("LLM Models SDK for Go\n")
	fmt.Printf("Total models: %d\n", len(Models))
	
	// Example usage
	openAIModels := GetModelsByProvider("OpenAI")
	fmt.Printf("OpenAI models: %d\n", len(openAIModels))
	
	functionCallingModels := GetFunctionCallingModels()
	fmt.Printf("Models with function calling: %d\n", len(functionCallingModels))
	
	// Print first model as JSON example
	if len(Models) > 0 {
		jsonData, _ := json.MarshalIndent(Models[0], "", "  ")
		fmt.Printf("\nFirst model:\n%s\n", jsonData)
	}
}
"#;

pub const GOMOD_TEMPLATE: &str = r#"module llm-models

go 1.21

require (
)
"#;