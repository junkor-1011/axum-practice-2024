.PHONY: build build-gnu build-musl build-aarch64 build-aarch64-musl debug-start start start-gnu start-musl stop clean test

target/x86_64-unknown-linux-gnu/release/practice1:
	cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.36 --target-dir target

build: target/x86_64-unknown-linux-gnu/release/practice1

build-gnu: target/x86_64-unknown-linux-gnu/release/practice1

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

start: build
	docker compose build; docker compose up -d

start-gnu: build-gnu
	docker compose -f compose-x86_64-gnu.yaml build; docker compose -f compose-x86_64-gnu.yaml up -d

start-musl: build-musl
	docker compose -f compose-x86_64-musl.yaml build; docker compose -f compose-x86_64-musl.yaml up -d

start-aarch64: build-aarch64
	docker compose -f compose-aarch64-gnu.yaml build; docker compose -f compose-aarch64-gnu.yaml up -d

start-aarch64-musl: build-aarch64-musl
	docker compose -f compose-aarch64-musl.yaml build; docker compose -f compose-aarch64-musl.yaml up -d

stop:
	docker compose down

clean:
	cargo clean
	rm -rf target

test:
	cargo test

# audit:
# 	cargo audit
