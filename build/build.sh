#!/bin/bash

TAG=""
TARGET_DIR=debug
if [ -n "${RELEASE}" ]; then
  TAG="--release"
  TARGET_DIR=release
fi

echo $TARGET_DIR build

rm -rf bin/executable
cargo build $TAG || { echo "build failed"; exit 1; }

mkdir -p bin/executable
mv ./target/$TARGET_DIR/i18n_locale_lint_cli bin/executable/i18n_locale_lint_cli
