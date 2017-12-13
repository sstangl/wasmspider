// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var benchmark = undefined;;
let importObject = undefined;

fetch("wasmspider.wasm").then(response =>
    response.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, importObject)
).then(result =>
    benchmark = result
);
