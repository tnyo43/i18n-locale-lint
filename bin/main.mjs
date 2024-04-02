#!/usr/bin/env node --experimental-wasi-unstable-preview1

import fs from "node:fs";
import { exit } from "node:process";
import { WASI } from "wasi";

const wasi = new WASI({
  version: "preview1",
  args: process.argv.slice(2),
  preopens: {
    ".": ".",
  },
});

const url = new URL("../wasm/i18n-locale-lint-cli.wasm", import.meta.url);
const wasm = await WebAssembly.compile(fs.readFileSync(url));
const importObject = { wasi_snapshot_preview1: wasi.wasiImport };
const instance = await WebAssembly.instantiate(wasm, importObject);

const statusCode = wasi.start(instance);
exit(statusCode);
