# The WasmSpider Benchmark Suite

This is a joke. Please do not actually use this for anything.

## Building

The only dependency is the Rust compiler with the wasm32-unknown-unknown target.

To build the project, execute the following:

1. `rustup target add wasm32-unknown-unknown`

2. `make run` (builds the project and opens it locally in Firefox.)

## Adding a test

Cargo-cult in the obvious way, keeping the same names as the SunSpider
originals. Replace dashes with underscores in the Rust code.

Then add the test to the list in `resources/sunspider-test-prefix.js`.
