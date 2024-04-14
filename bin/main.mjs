#!/usr/bin/env node

import { spawn } from "node:child_process";
import path from "node:path";
import { exit } from "node:process";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const EXE_FILE = "./executable/i18n_locale_lint_cli";
const args = process.argv;

const childProcess = spawn(path.resolve(__dirname, EXE_FILE), args.slice(2));

childProcess.stdout.on("data", (data) => {
  console.log(`${data}`);
});

childProcess.stderr.on("data", (data) => {
  console.error(`${data}`);
});

childProcess.on("close", (code) => {
  exit(code);
});
