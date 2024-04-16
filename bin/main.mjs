#!/usr/bin/env node --experimental-wasi-unstable-preview1

import { glob } from "glob";
import fs from "node:fs";
import { exit } from "node:process";
import { WASI } from "wasi";

const PROGRAM = "i18n-locale-lint";

let argv = process.argv.slice(2);
let globIndex = argv.findIndex((arg) => arg === "--glob");
if (globIndex !== -1) {
  const pattern = argv[globIndex + 1];
  const files = await glob(pattern, { ignore: "node_modules/**" });
  argv = argv
    .slice(0, globIndex)
    .concat(argv.slice(globIndex + 2))
    .concat(files);
}

const wasi = new WASI({
  version: "preview1",
  args: [PROGRAM, ...argv],
  preopens: {
    ".": ".",
  },
});

const url = new URL("../wasm/i18n_locale_lint_cli.wasm", import.meta.url);
const wasm = await WebAssembly.compile(fs.readFileSync(url));
const importObject = { wasi_snapshot_preview1: wasi.wasiImport };
const instance = await WebAssembly.instantiate(wasm, importObject);

const statusCode = wasi.start(instance);
exit(statusCode);
