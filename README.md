# i18n-locale-lint

This is package to checks JSON locale files to ensure that all keys are present in each file. It's particularly useful in multilingual applications to guarantee that all languages have the same set of keys.

# Install

```bash
$ npm i i18n-locale-lint
```

# use case

Add a script in your package.json.

```json
{
  "script": {
    "i18n-lint": "i18n-locale-lint \"./src/i18n/locales/**/*.json\""
  }
}
```

Run the script.

```bash
$ npm run i18n-lint
```
