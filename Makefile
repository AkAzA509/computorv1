# Makefile

.PHONY: all debugz

all:
	cargo build --release

debug:
	cargo run