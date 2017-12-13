# The WasmSpider Benchmark Suite

This is a joke. Please do not actually use this for anything.

## Building

You'll need a Nightly version of Rust, plus wasm-gc.

To get all those things, execute the following:

1. `rustup toolchain install nightly` (`rustup` can be obtained from https://www.rust-lang.org/.)

2. `rustup update`

3. `rustup target add wasm32-unknown-unknown --toolchain nightly`

4. `cargo install --git https://github.com/alexcrichton/wasm-gc`

5. `make run` (builds the project and opens it locally in Firefox.)

## Adding a test

Cargo-cult in the obvious way, keeping the same names as the SunSpider
originals. Replace dashes with underscores in the Rust code.

Then add the test to the list in `resources/sunspider-test-prefix.js`.
