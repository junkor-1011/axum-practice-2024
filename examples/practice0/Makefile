.PHONY: clean build debug-start

target/release/practice0:
	cargo build --release

build: target/release/practice0

debug-start:
	RUST_LOG=debug cargo run

clean:
	cargo clean
