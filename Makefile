.PHONY: run build test clippy fmt lint docker clean

run:
	cargo run --bin rust-lib-template -- "hello"

build:
	cargo build --release

test:
	cargo test --all

clippy:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all -- --check

lint: fmt clippy

docker:
	docker build -t rust-lib-template .

clean:
	cargo clean
	rm -rf bin/