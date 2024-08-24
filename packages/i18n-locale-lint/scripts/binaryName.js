const { execSync } = require("node:child_process");
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
    execSync("ldd --version", { stdio: ["pipe", "pipe", "pipe"] });
  } catch (err) {
    stderr = err;
  }
  return stderr?.toString()?.indexOf("musl") > -1;
}

const binName =
  platform === "linux" && isMusl()
    ? BIN_NAME_OF_PLATFORM["linux-musl"]?.[arch]
    : BIN_NAME_OF_PLATFORM[platform]?.[arch];

exports.binName = binName;
