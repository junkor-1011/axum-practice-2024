.PHONY: clean build debug-start

target/release/axum-practice-2024:
	cargo build --release

build: target/release/axum-practice-2024

debug-start:
	RUST_LOG=debug cargo run

clean:
	cargo clean
