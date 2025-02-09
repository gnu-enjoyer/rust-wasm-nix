build:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-name wasm_example \
    --out-dir examples/wasm/target \
    --target web target/wasm32-unknown-unknown/release/wasm-test.wasm

run: build
	python3 -m http.server --directory examples/wasm/target

.PHONY: run
