#!/usr/bin/env node

const path = require("node:path");
const child_process = require("node:child_process");

const binary_path = path.resolve(__dirname, "./main");
child_process.execFileSync(binary_path, process.argv.slice(2), {
  stdio: "inherit",
});
