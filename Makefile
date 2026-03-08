SHELL := /usr/bin/env bash

.PHONY: precommit-install precommit-run lint fmt clippy secret-scan

precommit-install:
	pre-commit install

precommit-run:
	pre-commit run --all-files

lint: fmt clippy

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

secret-scan:
	gitleaks detect --source . --no-banner
