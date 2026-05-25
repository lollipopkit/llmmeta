SHELL := /bin/sh

TMP_DIR ?= /private/tmp/llmmeta
MODELS_JSON ?= $(TMP_DIR)/models.json
NPM_CACHE ?= $(TMP_DIR)/npm-cache

.PHONY: all fmt fmt-check check test clippy ci fetch generate-rust generate-python generate-go generate-dart generate-typescript generate-all verify-generated clean-tmp

all: ci

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

check:
	cargo check

test:
	cargo test

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

ci: fmt-check check test clippy

fetch:
	mkdir -p $(TMP_DIR)
	cargo run -- fetch --output $(MODELS_JSON)

generate-rust: fetch
	cargo run -- generate --input $(MODELS_JSON) --lang rust --output $(TMP_DIR)/rust

generate-python: fetch
	cargo run -- generate --input $(MODELS_JSON) --lang python --output $(TMP_DIR)/python

generate-go: fetch
	cargo run -- generate --input $(MODELS_JSON) --lang go --output $(TMP_DIR)/go

generate-dart: fetch
	cargo run -- generate --input $(MODELS_JSON) --lang dart --output $(TMP_DIR)/dart

generate-typescript: fetch
	cargo run -- generate --input $(MODELS_JSON) --lang typescript --output $(TMP_DIR)/typescript

generate-all: generate-rust generate-python generate-go generate-dart generate-typescript

verify-generated: generate-all
	cargo test --manifest-path $(TMP_DIR)/rust/Cargo.toml
	python3 -m py_compile $(TMP_DIR)/python/src/llm_models/models.py $(TMP_DIR)/python/src/llm_models/__init__.py
	cd $(TMP_DIR)/go && go test ./...
	dart analyze $(TMP_DIR)/dart
	cd $(TMP_DIR)/typescript && npm install --cache $(NPM_CACHE) && npm run build

clean-tmp:
	rm -rf $(TMP_DIR)
