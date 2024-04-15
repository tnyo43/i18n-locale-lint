# i18n-locale-lint

This is a package to check JSON/YAML locale files to ensure that all keys are present in each file. It's particularly useful in multilingual applications to guarantee that all languages have the same set of keys.

# Run

If you have your locale files at "src/locale" directory, run:

```bash
$ npx run i18n-locale-lint ./src/locale/**/*.json
```

Or if you define an npm script with this library, it's recommended to enclose the glob pattern within double quotes as like below.

```json
{
  "scripts": {
    "i18n-lint": "i18n-locale-lint \"./src/locale/**/*.json\""
  }
}
```

## Options

| Flag               | Short Flag | Description                                                                   |
| ------------------ | ---------- | ----------------------------------------------------------------------------- |
| `--silent`         | `-s`       | Don't display logs other than errors.                                         |
| `--skip-top-level` |            | Assuming the top level is composed solely of a single key, skip it.           |
| `--grouped-by`     | `-g`       | Group locale file by a regular expression. (`-g "^(.*/)([^/]+)$"` by default) |
