#!/usr/bin/env node

const path = require("node:path");
const child_process = require("node:child_process");
const { binName } = require("../scripts/binaryName");

const binPath = path.resolve(__dirname, binName);
const result = child_process.spawnSync(binPath, process.argv.slice(2), {
  stdio: "inherit",
});
if (result.error) {
  throw result.error;
}

process.exitCode = result.status;
