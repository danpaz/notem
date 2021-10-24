# Thanks to @PureW for the Makefile template.
# Makefile styleguide at http://clarkgrubb.com/makefile-style-guide
MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := all
.DELETE_ON_ERROR:
.SUFFIXES:

RUST_VERSION := $(shell cat .rust-version)
RUSTFLAGS ?=-g
CARGO_LOCKED:=--locked
CARGO:=RUSTFLAGS="$(RUSTFLAGS) --deny warnings" cargo +$(RUST_VERSION) $(CARGO_LOCKED)

.PHONY: all
all:
	@echo "Building $@"
	$(CARGO) build --release

.PHONY: fmt
fmt:
	@echo "Executing $@"
	${CARGO} fmt --all

.PHONY: test
test:
	@echo "Building $@"
	make test-rust
	make test-fmt
	make test-clippy

.PHONY: test-rust
test-rust:
	@echo "Building $@"
	$(CARGO) check
	$(CARGO) test
	$(CARGO) build

.PHONY: test-fmt
test-fmt:
	@echo "Building $@"
	$(CARGO) fmt --all -- --check

# Put clippy artifacts in separate CARGO_TARGET_DIR to speed up compilation
CLIPPY_ARTIFACTS := $(HOME)/.cargo/clippy-build-artifacts
.PHONY: test-clippy
test-clippy:
	@echo "Building $@"
	CARGO_TARGET_DIR=$(CLIPPY_ARTIFACTS) $(CARGO) clippy --all-features --all-targets -- -W clippy::all -D warnings

.PHONY: rust-update
rust-update:
	@echo "Executing $@"
	rustup toolchain install $(RUST_VERSION)
	rustup component add --toolchain $(RUST_VERSION) rustfmt
	rustup component add --toolchain $(RUST_VERSION) clippy

.PHONY: run
run:
	@echo "Executing $@"
	${CARGO} run