#!/usr/bin/env node

const path = require("node:path");
const { execSync, spawnSync } = require("node:child_process");
const { platform, arch } = process;

const BIN_NAME_OF_PLATFORM = {
  darwin: {
    x64: "main-x86_64-apple-darwin",
    arm64: "main-aarch64-apple-darwin",
  },
  linux: {
    x64: "main-x86_64-unknown-linux-gnu",
    arm64: "main-aarch64-unknown-linux-gnu",
  },
  "linux-musl": {
    x64: "main-x86_64-unknown-linux-musl",
    arm64: "main-aarch64-unknown-linux-musl",
  },
};

function isMusl() {
  let stderr;
  try {
    stderr = execSync("ldd --version", {
      stdio: ["pipe", "pipe", "pipe"],
    });
  } catch (err) {
    stderr = err.stderr;
  }
  if (stderr?.indexOf("musl") > -1) {
    return true;
  }
  return false;
}

const binName =
  platform === "linux" && isMusl()
    ? BIN_NAME_OF_PLATFORM["linux-musl"]?.[arch]
    : BIN_NAME_OF_PLATFORM[platform]?.[arch];

const binary_path = path.resolve(__dirname, "executable/", binName);
const result = spawnSync(binary_path, process.argv.slice(2), {
  stdio: "inherit",
});
if (result.error) {
  throw result.error;
}

process.exitCode = result.status;
