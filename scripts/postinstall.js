const fs = require("node:fs");
const path = require("node:path");
const https = require("follow-redirects/https");
const { exit } = require("node:process");
const { execSync } = require("node:child_process");

const DISTRIBUTION_VERSION = require("../package.json").version;
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

if (!binName) {
  console.warn(
    "This package doesn't ship with prebuilt binaries for your platform yet."
  );
  exit(1);
}

const PATH_EXECUTABLE = "bin/executable";
const PATH_EXECUTABLE_FILE = path.resolve(PATH_EXECUTABLE, binName);

async function downloadAssetUrl() {
  const result = await new Promise((resolve, reject) => {
    const url = new URL(
      `https://api.github.com/repos/tnyo43/i18n-locale-lint/releases/tags/v${DISTRIBUTION_VERSION}`
    );
    const hostname = url.hostname;
    const path = url.pathname;
    https
      .request(
        {
          hostname,
          path,
          headers: {
            Accept: "application/json",
            "User-Agent": "@tnyo43/i18n-locale-lint",
            "X-GitHub-Api-Version": "2022-11-28",
          },
        },
        (res) => {
          let data = "";
          res.on("data", (chunk) => {
            data += chunk;
          });
          res.on("end", () => {
            resolve(JSON.parse(data));
          });
        }
      )
      .on("error", (e) => {
        reject(e);
      })
      .end();
  });

  const asset = result.assets.find((value) => value.name === binName);
  return asset.url;
}

async function downloadBinary(urlBase, filePath) {
  const url = new URL(urlBase);
  const hostname = url.hostname;
  const path = url.pathname;

  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(filePath);
    let fileInfo = null;

    const request = https.request(
      {
        hostname,
        path,
        headers: {
          Accept: "application/octet-stream",
          "User-Agent": "@tnyo43/i18n-locale-lint",
          "X-GitHub-Api-Version": "2022-11-28",
        },
      },
      (response) => {
        if (response.statusCode !== 200) {
          fs.unlink(filePath, () => {
            reject(
              new Error(`Failed to get '${url}' (${response.statusCode})`)
            );
          });
          return;
        }

        fileInfo = {
          mime: response.headers["content-type"],
          size: parseInt(response.headers["content-length"], 10),
        };
        response.pipe(file);
      }
    );

    file.on("finish", () => resolve(fileInfo));
    file.on("error", (err) => {
      console.log("file error", err);
      fs.unlink(filePath, () => reject(err));
    });

    request.end();
  });
}

function makeDirectory() {
  fs.mkdirSync(PATH_EXECUTABLE, { recursive: true });
}

async function main() {
  makeDirectory();
  const urlBase = await downloadAssetUrl();
  await downloadBinary(urlBase, PATH_EXECUTABLE_FILE);
  fs.chmodSync(PATH_EXECUTABLE_FILE, "755");
}

main();
