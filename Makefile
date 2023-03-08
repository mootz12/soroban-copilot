default: build

test: build
	cargo test --all --tests

build:
	cargo build

build-contracts:
	cargo build --target wasm32-unknown-unknown --release -p token

generate-wasm: build-contracts
	soroban contract optimize \
		--wasm target/wasm32-unknown-unknown/release/token.wasm \
		--wasm-out soroban-contracts/wasm/token.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
