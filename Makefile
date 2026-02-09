# I NEED MY MAKEFILE

.PHONY: run build release check test clean fmt clippy

run:
	cargo run

build:
	cargo build

release:
	cargo build --release

check:
	cargo check

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

clean:
	cargo clean
