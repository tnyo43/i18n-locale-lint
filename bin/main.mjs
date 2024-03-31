#!/usr/bin/env node

import fs from "node:fs";
import { exit } from "node:process";
import { glob } from "glob";
import { WASI } from "wasi";

const pattern = process.argv[2];
const files = await glob(pattern);

const wasi = new WASI({
  version: "preview1",
  args: [files],
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
