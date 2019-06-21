.PHONY: builddir wasm resources run

all: wasm resources
run: wasm resources launch

builddir:
	mkdir -p build

wasm: builddir
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/wasmspider.wasm build/

resources: builddir
	cp resources/* build/

launch:
	firefox build/index.html
