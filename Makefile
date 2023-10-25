build:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --out-name wasm_example \
    --out-dir examples/wasm/target \
    --target web target/wasm32-unknown-unknown/debug/wasm-test.wasm

debug: build
	python3 -m http.server --directory examples/wasm/target

.PHONY: debug
