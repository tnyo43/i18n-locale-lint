#!/usr/bin/env node

const path = require("node:path");
const child_process = require("node:child_process");

const binary_path = path.resolve(__dirname, "./executable/main");
const result = child_process.spawnSync(binary_path, process.argv.slice(2), {
  stdio: "inherit",
});
if (result.error) {
  throw result.error;
}

process.exitCode = result.status;
