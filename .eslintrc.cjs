module.exports = {
  root: true,
  extends: [
    'eslint:recommended'
  ],
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
        "indent": ["error", 2, { "SwitchCase": 1 }],
        "linebreak-style": ["error", "unix"],
        "quotes": ["error", "single"],
        "comma-dangle": ["error", "always-multiline"],
        "@typescript-eslint/no-explicit-any": 0
      }
    }
  ]
};