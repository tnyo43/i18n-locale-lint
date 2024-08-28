const { execSync } = require("node:child_process");
const { platform, arch } = process;

const BIN_NAME_OF_PLATFORM = {
  darwin: {
    x64: "@i18n-locale-lint/cli-darwin-x64/main",
    arm64: "@i18n-locale-lint/cli-darwin-arm64/main",
  },
  linux: {
    x64: "@i18n-locale-lint/cli-linux-x64/main",
    arm64: "@i18n-locale-lint/cli-linux-arm64/main",
  },
  "linux-musl": {
    x64: "@i18n-locale-lint/cli-linux-x64-musl/main",
    arm64: "@i18n-locale-lint/cli-linux-arm64-musl/main",
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
