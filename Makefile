# This Makefile is not for building the project, use cargo instead
# this is only a helper for generating the lemon parser

LEMON_RUST := lemon_rust
TEMPLATE = src/_grammar.rs

.PHONY: all
all: src/grammar.rs
	cargo build

%.rs: %.y $(TEMPLATE)
	$(LEMON_RUST) -s -T$(TEMPLATE) $<
