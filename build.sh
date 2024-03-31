#!/bin/bash

(
    cd crates/cli
    cargo wasi build
)
mkdir -p wasm
mv ./target/wasm32-wasi/debug/json-inspector-cli.wasm wasm/json-inspector-cli.wasm
