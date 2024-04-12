#!/bin/bash

TAG=""
TARGET_DIR=debug
if [ -n "${RELEASE}" ]; then
  TAG="--release"
  TARGET_DIR=release
fi

echo $TARGET_DIR build

rm -rf wasm
cargo wasi build $TAG || { echo "build failed"; exit 1; }

mkdir -p wasm
mv ./target/wasm32-wasi/$TARGET_DIR/i18n-locale-lint-cli.wasm wasm/i18n-locale-lint-cli.wasm
