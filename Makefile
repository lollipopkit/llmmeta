SHELL := /bin/sh

TMP_DIR ?= $(or $(TMPDIR),$(TMP),/tmp)/llmmeta
MODELS_JSON ?= $(TMP_DIR)/models.json
NPM_CACHE ?= $(TMP_DIR)/npm-cache
GO_MODULE ?= github.com/lollipopkit/llmmeta/sdks/go

.PHONY: all fmt fmt-check check test clippy ci fetch generate-rust generate-python generate-go generate-dart generate-typescript generate-all verify-generated verify-publish-generated verify-go-import clean-tmp

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

generate-rust:
	cargo run -- generate --input $(MODELS_JSON) --lang rust --output $(TMP_DIR)/rust

generate-python:
	cargo run -- generate --input $(MODELS_JSON) --lang python --output $(TMP_DIR)/python

generate-go:
	cargo run -- generate --input $(MODELS_JSON) --lang go --output $(TMP_DIR)/go --go-module $(GO_MODULE)

generate-dart:
	cargo run -- generate --input $(MODELS_JSON) --lang dart --output $(TMP_DIR)/dart

generate-typescript:
	cargo run -- generate --input $(MODELS_JSON) --lang typescript --output $(TMP_DIR)/typescript

generate-all: fetch
	$(MAKE) generate-rust generate-python generate-go generate-dart generate-typescript

verify-generated: generate-all
	cargo test --manifest-path $(TMP_DIR)/rust/Cargo.toml
	python3 -m py_compile $(TMP_DIR)/python/src/llm_models/models.py $(TMP_DIR)/python/src/llm_models/__init__.py
	cd $(TMP_DIR)/go && go test ./...
	$(MAKE) verify-go-import
	dart analyze $(TMP_DIR)/dart
	cd $(TMP_DIR)/typescript && npm install --cache $(NPM_CACHE) && npm run build

verify-go-import:
	rm -rf $(TMP_DIR)/go-consumer
	mkdir -p $(TMP_DIR)/go-consumer
	cd $(TMP_DIR)/go-consumer && go mod init example.com/llmmeta-go-consumer
	cd $(TMP_DIR)/go-consumer && go mod edit -require $(GO_MODULE)@v0.0.0
	cd $(TMP_DIR)/go-consumer && go mod edit -replace $(GO_MODULE)=$(TMP_DIR)/go
	printf 'package main\n\nimport (\n\t"fmt"\n\tllmmeta "$(GO_MODULE)"\n)\n\nfunc main() {\n\tfmt.Println(len(llmmeta.GetAllModels()))\n}\n' > $(TMP_DIR)/go-consumer/main.go
	cd $(TMP_DIR)/go-consumer && go run .

verify-publish-generated: verify-generated
	cd $(TMP_DIR)/rust && cargo publish --dry-run --allow-dirty
	cd $(TMP_DIR)/dart && dart pub publish --dry-run
	cd $(TMP_DIR)/typescript && npm pack --dry-run --cache $(NPM_CACHE)

clean-tmp:
	rm -rf $(TMP_DIR)
