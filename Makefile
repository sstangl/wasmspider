.PHONY: builddir wasm resources run

all: wasm resources
run: wasm resources launch

builddir:
	mkdir -p build

wasm: builddir
	cargo +nightly build --target wasm32-unknown-unknown --release
	wasm-gc target/wasm32-unknown-unknown/release/wasmspider.wasm build/wasmspider.wasm

resources: builddir
	cp resources/* build/

launch:
	firefox build/index.html
