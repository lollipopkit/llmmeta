pub const MODELS_TEMPLATE: &str = r#"/// LLM Models from models.dev
/// Generated at: {{timestamp}}
/// Total models: {{model_count}}

import 'dart:convert';

class Model {
  final String id;
  final String name;
  final String? description;
  final int? contextLength;
  final int? outputLength;
  final double? inputCost;
  final double? outputCost;
  final String? releaseDate;
  final String? knowledgeCutoff;
  final List<String> modalities;
  final bool? reasoning;
  final bool? functionCalling;
  final bool? toolUse;
  final bool? openWeight;
  final Provider provider;

  const Model({
    required this.id,
    required this.name,
    this.description,
    this.contextLength,
    this.outputLength,
    this.inputCost,
    this.outputCost,
    this.releaseDate,
    this.knowledgeCutoff,
    required this.modalities,
    this.reasoning,
    this.functionCalling,
    this.toolUse,
    this.openWeight,
    required this.provider,
  });

  bool supportsModality(String modality) {
    return modalities.contains(modality);
  }

  (double, double)? getPricing() {
    if (inputCost != null && outputCost != null) {
      return (inputCost!, outputCost!);
    }
    return null;
  }

  bool supportsFunctionCalling() {
    return (functionCalling ?? false) || (toolUse ?? false);
  }

  bool hasReasoning() {
    return reasoning ?? false;
  }

  bool isOpenSource() {
    return openWeight ?? false;
  }

  factory Model.fromJson(Map<String, dynamic> json) {
    return Model(
      id: json['id'],
      name: json['name'],
      description: json['description'],
      contextLength: json['context_length'],
      outputLength: json['output_length'],
      inputCost: (json['input_cost'] as num?)?.toDouble(),
      outputCost: (json['output_cost'] as num?)?.toDouble(),
      releaseDate: json['release_date'],
      knowledgeCutoff: json['knowledge_cutoff'],
      modalities: List<String>.from(json['modalities'] ?? const []),
      reasoning: json['reasoning'],
      functionCalling: json['function_calling'],
      toolUse: json['tool_use'],
      openWeight: json['open_weight'],
      provider: Provider.fromJson(json['provider']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'description': description,
      'context_length': contextLength,
      'output_length': outputLength,
      'input_cost': inputCost,
      'output_cost': outputCost,
      'release_date': releaseDate,
      'knowledge_cutoff': knowledgeCutoff,
      'modalities': modalities,
      'reasoning': reasoning,
      'function_calling': functionCalling,
      'tool_use': toolUse,
      'open_weight': openWeight,
      'provider': provider.toJson(),
    };
  }
}

class Provider {
  final String id;
  final String name;
  final String? description;
  final String? website;

  const Provider({
    required this.id,
    required this.name,
    this.description,
    this.website,
  });

  factory Provider.fromJson(Map<String, dynamic> json) {
    return Provider(
      id: json['id'],
      name: json['name'],
      description: json['description'],
      website: json['website'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'description': description,
      'website': website,
    };
  }
}

const String _modelsJson = {{{models_json_literal_dart}}};

List<Model> _loadModels() {
  final decoded = jsonDecode(_modelsJson) as List<dynamic>;
  return List.unmodifiable(decoded
      .map((item) => Model.fromJson(item as Map<String, dynamic>))
      .toList(growable: false));
}

/// All available models
final List<Model> models = _loadModels();
"#;

pub const LIB_TEMPLATE: &str = r#"/// LLM Models SDK for Dart
/// Generated from models.dev at: {{timestamp}}

library llm_models;

export 'models.dart';

import 'models.dart';

/// Get all models
List<Model> getAllModels() {
  return models;
}

/// Get models by provider name
List<Model> getModelsByProvider(String providerName) {
  return models.where((model) => model.provider.name == providerName).toList();
}

/// Get models that support a specific modality
List<Model> getModelsByModality(String modality) {
  return models.where((model) => model.supportsModality(modality)).toList();
}

/// Get models with function calling support
List<Model> getFunctionCallingModels() {
  return models.where((model) => model.supportsFunctionCalling()).toList();
}

/// Get models with reasoning capabilities
List<Model> getReasoningModels() {
  return models.where((model) => model.hasReasoning()).toList();
}

/// Get open source models
List<Model> getOpenSourceModels() {
  return models.where((model) => model.isOpenSource()).toList();
}

/// Get models sorted by price (low to high)
List<Model> getModelsSortedByPrice() {
  var modelsWithPricing = models.where((model) => model.getPricing() != null).toList();
  modelsWithPricing.sort((a, b) {
    var pricingA = a.getPricing()!;
    var pricingB = b.getPricing()!;
    var totalA = pricingA.$1 + pricingA.$2;
    var totalB = pricingB.$1 + pricingB.$2;
    return totalA.compareTo(totalB);
  });
  return modelsWithPricing;
}
"#;

pub const PUBSPEC_TEMPLATE: &str = r#"name: {{package_name}}
version: {{package_version}}
description: LLM models data from models.dev
homepage: {{package_homepage}}
repository: {{repository}}
issue_tracker: {{package_issues}}
topics:
  - llm
  - ai
  - models
  - metadata

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  meta: ^1.9.1

dev_dependencies:
  test: ^1.24.0
"#;
