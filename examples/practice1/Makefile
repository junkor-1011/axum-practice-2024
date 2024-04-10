.PHONY: build build-musl build-aarch64 build-aarch64-musl debug-start clean

target/x86_64-unknown-linux-gnu/release/practice1:
	cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.36 --target-dir target

build: target/x86_64-unknown-linux-gnu/release/practice1

target/x86_64-unknown-linux-musl/release/practice1:
	cargo zigbuild --release --target x86_64-unknown-linux-musl --target-dir target

build-musl: target/x86_64-unknown-linux-musl/release/practice1

target/aarch64-unknown-linux-gnu/release/practice1:
	cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.36 --target-dir target

build-aarch64: target/aarch64-unknown-linux-gnu/release/practice1

target/aarch64-unknown-linux-musl/release/practice1:
	cargo zigbuild --release --target aarch64-unknown-linux-musl --target-dir target

build-aarch64-musl: target/aarch64-unknown-linux-musl/release/practice1

debug-start:
	RUST_LOG=debug cargo run --target-dir target

clean:
	# cargo clean
	rm -rf target
