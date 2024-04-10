.PHONY: clean build debug-start

target/release/practice0:
	cargo build --release --target-dir target

build: target/release/practice0

debug-start:
	RUST_LOG=debug cargo run --target-dir target

clean:
	# cargo clean
	rm -rf target
