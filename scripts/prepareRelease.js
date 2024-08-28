const fs = require("node:fs");
const https = require("follow-redirects/https");
const { exit } = require("node:process");

const PLATFORMS = ["darwin", "linux", "linux-musl"];
const ARCHITECTURES = ["x64", "arm64"];

const BIN_AND_PATH_NAME_OF_PLATFORM = {
  darwin: {
    x64: {
      bin: "main-x86_64-apple-darwin",
      path: "cli-darwin-x64/main",
    },
    arm64: {
      bin: "main-aarch64-apple-darwin",
      path: "cli-darwin-arm64/main",
    },
  },
  linux: {
    x64: {
      bin: "main-x86_64-unknown-linux-gnu",
      path: "cli-linux-x64/main",
    },
    arm64: {
      bin: "main-aarch64-unknown-linux-gnu",
      path: "cli-linux-arm64/main",
    },
  },
  "linux-musl": {
    x64: {
      bin: "main-x86_64-unknown-linux-musl",
      path: "cli-linux-x64-musl/main",
    },
    arm64: {
      bin: "main-aarch64-unknown-linux-musl",
      path: "cli-linux-arm64-musl/main",
    },
  },
};

const DISTRIBUTION_VERSION = require("../package.json").version;
console.log(`distribution version: ${DISTRIBUTION_VERSION}`);
if (typeof DISTRIBUTION_VERSION !== "string") {
  console.warn(`The asset list version is wrong`);
  exit(1);
}

async function downloadAssetUrls() {
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

  return result.assets;
}

async function downloadBinary(urlBase, packageName) {
  const url = new URL(urlBase);
  const hostname = url.hostname;
  const path = url.pathname;

  const filePath = `packages/${packageName}`;

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

    file.on("finish", () => {
      resolve(fileInfo);
      fs.chmodSync(filePath, "755");
    });
    file.on("error", (err) => {
      console.log("file error", err);
      fs.unlink(filePath, () => reject(err));
    });

    request.end();
  });
}

async function main() {
  const assets = await downloadAssetUrls();

  for (platform of PLATFORMS) {
    for (arch of ARCHITECTURES) {
      console.log(
        "download",
        BIN_AND_PATH_NAME_OF_PLATFORM[platform][arch].bin,
        BIN_AND_PATH_NAME_OF_PLATFORM[platform][arch].path
      );
      const asset = assets.find(
        (asset) =>
          asset.name === BIN_AND_PATH_NAME_OF_PLATFORM[platform][arch].bin
      );
      if (!asset) {
        console.warn(
          `can't find asset ${BIN_AND_PATH_NAME_OF_PLATFORM[platform][arch].bin}`
        );
      } else {
        await downloadBinary(
          asset.url,
          BIN_AND_PATH_NAME_OF_PLATFORM[platform][arch].path
        );
      }
    }
  }
}

main();
