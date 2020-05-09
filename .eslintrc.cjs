module.exports = {
  root: true,
  extends: [
    'eslint:recommended'
  ],
  "env": {
    "es2020": true,
    "browser": true,
    "node": true
  },
  "parserOptions": {
    "sourceType": "module",
  },
  "overrides": [
    {
      "files": ["**/*.ts"],
      "env": {
        "browser": true,
        "es6": true,
        "node": true
      },
      "extends": [
        "eslint:recommended",
        "plugin:@typescript-eslint/eslint-recommended",
        "plugin:@typescript-eslint/recommended"
      ],
      "globals": {
        "Atomics": "readonly",
        "SharedArrayBuffer": "readonly"
      },
      "parser": "@typescript-eslint/parser",
      "parserOptions": {
        "ecmaVersion": 2018,
        "sourceType": "module",
        "project": "./tsconfig.json"
      },
      "plugins": ["@typescript-eslint"],
      "rules": {
        "indent": ["warn", 2, { "SwitchCase": 1 }],
        "linebreak-style": ["warn", "unix"],
        "quotes": ["warn", "single"],
        "comma-dangle": ["warn", "always-multiline"],
        "@typescript-eslint/no-explicit-any": "warn"
      }
    }
  ]
}