const fs = require("node:fs");
const path = require("node:path");
const https = require("follow-redirects/https");
const { exit } = require("node:process");
const { binName } = require("./binaryName");

const PATH_EXECUTABLE = "bin";
const PATH_EXECUTABLE_FILE = path.resolve(PATH_EXECUTABLE, binName);

const DISTRIBUTION_VERSION = require("../package.json").version;

if (!binName) {
  console.warn(
    "This package doesn't ship with prebuilt binaries for your platform yet."
  );
  exit(1);
}

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

async function main() {
  const urlBase = await downloadAssetUrl();
  await downloadBinary(urlBase, PATH_EXECUTABLE_FILE);
  fs.chmodSync(PATH_EXECUTABLE_FILE, "755");
}

main();
