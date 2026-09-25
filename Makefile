# Makefile

.PHONY: all debug

all:
	cargo build --release

debug:
	cargo run