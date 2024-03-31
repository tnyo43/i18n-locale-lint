#!/bin/bash

TAG=""
TARGET_DIR=debug
if [ -n "${RELEASE}" ]; then
  TAG="--release"
  TARGET_DIR=release
fi

echo $TARGET_DIR build

(
    cd crates/cli
    cargo wasi build $TAG
)
mkdir -p wasm
mv ./target/wasm32-wasi/$TARGET_DIR/json-inspector-cli.wasm wasm/json-inspector-cli.wasm
