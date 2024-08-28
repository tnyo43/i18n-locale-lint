#!/usr/bin/env node

const child_process = require("node:child_process");
const { binName } = require("../scripts/binaryName");

const result = child_process.spawnSync(
  require.resolve(binName),
  process.argv.slice(2),
  { stdio: "inherit" }
);
if (result.error) {
  throw result.error;
}

process.exitCode = result.status;
