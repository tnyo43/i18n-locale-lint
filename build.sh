#!/bin/bash

(
    cd crates/cli
    cargo wasi build
)
mkdir wasm
mv ./target/wasm32-wasi/debug/json-inspector-cli.wasm wasm/
