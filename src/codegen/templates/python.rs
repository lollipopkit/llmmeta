pub const MODELS_TEMPLATE: &str = r#""""LLM Models from models.dev.
Generated at: {{timestamp}}
Total models: {{model_count}}
"""

from dataclasses import dataclass
import json
from typing import Dict, List, Optional, Tuple


@dataclass
class Provider:
    """Model provider information."""
    id: str
    name: str
    description: Optional[str] = None
    website: Optional[str] = None

    @classmethod
    def from_dict(cls, data: Dict) -> "Provider":
        return cls(**data)

    def to_dict(self) -> Dict:
        return {
            "id": self.id,
            "name": self.name,
            "description": self.description,
            "website": self.website,
        }


@dataclass
class Model:
    """AI model information."""
    id: str
    name: str
    provider: Provider
    modalities: List[str]
    description: Optional[str] = None
    context_length: Optional[int] = None
    output_length: Optional[int] = None
    input_cost: Optional[float] = None
    output_cost: Optional[float] = None
    release_date: Optional[str] = None
    knowledge_cutoff: Optional[str] = None
    reasoning: Optional[bool] = None
    function_calling: Optional[bool] = None
    tool_use: Optional[bool] = None
    open_weight: Optional[bool] = None

    def supports_modality(self, modality: str) -> bool:
        """Check if model supports a specific modality."""
        return modality in self.modalities

    def get_pricing(self) -> Optional[Tuple[float, float]]:
        """Get input and output pricing."""
        if self.input_cost is not None and self.output_cost is not None:
            return (self.input_cost, self.output_cost)
        return None

    def supports_function_calling(self) -> bool:
        """Check if model supports function calling."""
        return bool(self.function_calling) or bool(self.tool_use)

    def has_reasoning(self) -> bool:
        """Check if model has reasoning capabilities."""
        return bool(self.reasoning)

    def is_open_source(self) -> bool:
        """Check if model is open source."""
        return bool(self.open_weight)

    @classmethod
    def from_dict(cls, data: Dict) -> "Model":
        """Create model from dictionary."""
        data = dict(data)
        provider = Provider.from_dict(data.pop("provider"))
        return cls(provider=provider, **data)

    def to_dict(self) -> Dict:
        """Convert model to dictionary."""
        return {
            "id": self.id,
            "name": self.name,
            "description": self.description,
            "context_length": self.context_length,
            "output_length": self.output_length,
            "input_cost": self.input_cost,
            "output_cost": self.output_cost,
            "release_date": self.release_date,
            "knowledge_cutoff": self.knowledge_cutoff,
            "modalities": self.modalities,
            "reasoning": self.reasoning,
            "function_calling": self.function_calling,
            "tool_use": self.tool_use,
            "open_weight": self.open_weight,
            "provider": self.provider.to_dict(),
        }


_MODELS_JSON = {{{models_json_literal}}}

# All available models
MODELS: List[Model] = [Model.from_dict(item) for item in json.loads(_MODELS_JSON)]
"#;

pub const INIT_TEMPLATE: &str = r#""""LLM Models SDK for Python.
Generated from models.dev at: {{timestamp}}
"""

from typing import List, Optional

from .models import Model, Provider, MODELS

__version__ = "0.1.0"
__all__ = ["Model", "Provider", "MODELS", "get_all_models", "get_models_by_provider",
           "get_models_by_modality", "get_function_calling_models", "get_reasoning_models",
           "get_open_source_models", "get_models_sorted_by_price"]


def get_all_models() -> List[Model]:
    """Get all available models."""
    return MODELS


def get_models_by_provider(provider_name: str) -> List[Model]:
    """Get models from a specific provider."""
    return [model for model in MODELS if model.provider.name == provider_name]


def get_models_by_modality(modality: str) -> List[Model]:
    """Get models that support a specific modality."""
    return [model for model in MODELS if model.supports_modality(modality)]


def get_function_calling_models() -> List[Model]:
    """Get models that support function calling."""
    return [model for model in MODELS if model.supports_function_calling()]


def get_reasoning_models() -> List[Model]:
    """Get models with reasoning capabilities."""
    return [model for model in MODELS if model.has_reasoning()]


def get_open_source_models() -> List[Model]:
    """Get open source models."""
    return [model for model in MODELS if model.is_open_source()]


def get_models_sorted_by_price() -> List[Model]:
    """Get models sorted by total price (low to high)."""
    models_with_pricing = [model for model in MODELS if model.get_pricing() is not None]
    return sorted(models_with_pricing, key=lambda model: sum(model.get_pricing()))


def search_models(query: str) -> List[Model]:
    """Search models by name or description."""
    query_lower = query.lower()
    return [
        model for model in MODELS
        if query_lower in model.name.lower() or
           (model.description and query_lower in model.description.lower())
    ]


def get_providers() -> List[str]:
    """Get all unique provider names."""
    return sorted(set(model.provider.name for model in MODELS))


def get_model_by_id(model_id: str) -> Optional[Model]:
    """Get a specific model by ID."""
    for model in MODELS:
        if model.id == model_id:
            return model
    return None
"#;

pub const PYPROJECT_TEMPLATE: &str = r#"[project]
name = "llm-models"
version = "0.1.0"
description = "LLM models data from models.dev"
readme = "README.md"
license = { text = "MIT" }
authors = [
    { name = "LLM Meta" },
]
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.8",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Topic :: Software Development :: Libraries :: Python Modules",
]
requires-python = ">=3.8"
dependencies = []

[project.urls]
Homepage = "https://github.com/your-username/llm-models-python"
Repository = "https://github.com/your-username/llm-models-python.git"
Issues = "https://github.com/your-username/llm-models-python/issues"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.hatch.build.targets.wheel]
packages = ["src/llm_models"]
"#;
