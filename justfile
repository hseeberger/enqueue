set shell := ["bash", "-uc"]

check:
	cargo check --tests

fmt:
	cargo +nightly fmt

lint:
	cargo clippy --no-deps -- -D warnings

test:
	cargo test

all: fmt check lint test
