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
};

const binName = BIN_NAME_OF_PLATFORM[platform]?.[arch];

exports.binName = binName;
