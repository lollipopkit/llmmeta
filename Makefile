SHELL := /bin/sh

TMP_DIR ?= $(or $(TMPDIR),$(TMP),/tmp)/llmmeta
MODELS_JSON ?= $(TMP_DIR)/models.json
NPM_CACHE ?= $(TMP_DIR)/npm-cache
GO_MODULE ?= github.com/lollipopkit/llmmeta/sdks/go
PYTHON_IMPORT_PACKAGE ?= llm_meta

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
	rm -rf $(TMP_DIR)/rust
	cargo run -- generate --input $(MODELS_JSON) --lang rust --output $(TMP_DIR)/rust

generate-python:
	rm -rf $(TMP_DIR)/python
	cargo run -- generate --input $(MODELS_JSON) --lang python --output $(TMP_DIR)/python

generate-go:
	rm -rf $(TMP_DIR)/go
	cargo run -- generate --input $(MODELS_JSON) --lang go --output $(TMP_DIR)/go --go-module $(GO_MODULE)

generate-dart:
	rm -rf $(TMP_DIR)/dart
	cargo run -- generate --input $(MODELS_JSON) --lang dart --output $(TMP_DIR)/dart

generate-typescript:
	rm -rf $(TMP_DIR)/typescript
	cargo run -- generate --input $(MODELS_JSON) --lang typescript --output $(TMP_DIR)/typescript

generate-all: fetch
	$(MAKE) generate-rust generate-python generate-go generate-dart generate-typescript

verify-generated: generate-all
	cargo test --manifest-path $(TMP_DIR)/rust/Cargo.toml
	python3 -m py_compile $(TMP_DIR)/python/src/$(PYTHON_IMPORT_PACKAGE)/models.py $(TMP_DIR)/python/src/$(PYTHON_IMPORT_PACKAGE)/__init__.py
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
	sed 's|{{GO_MODULE}}|$(GO_MODULE)|g' tests/fixtures/go-consumer-main.go.in > $(TMP_DIR)/go-consumer/main.go
	cd $(TMP_DIR)/go-consumer && go run .

verify-publish-generated: verify-generated
	cd $(TMP_DIR)/rust && cargo publish --dry-run --allow-dirty
	cd $(TMP_DIR)/python && python3 -m pip install build twine && python3 -m build --outdir dist && python3 -m twine check dist/*
	cd $(TMP_DIR)/dart && dart pub publish --dry-run
	cd $(TMP_DIR)/typescript && npm pack --dry-run --cache $(NPM_CACHE)

clean-tmp:
	rm -rf $(TMP_DIR)
