const path = require("path");

/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true,
  },
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/recommended-requiring-type-checking",
    "plugin:react/recommended",
    "plugin:eslint-plugin-import/recommended",
    "next/core-web-vitals",
    "prettier",
  ],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaVersion: "latest",
    project: path.resolve(__dirname, "./tsconfig.json"),
    sourceType: "module",
  },
  ignorePatterns: [
    "/.eslintrc.js",
    "package-lock.json",
    "package.json",
    "src/__generated__/**/*",
    "src/**/*.css.d.ts",
  ],
  plugins: ["import", "unused-imports"],
  rules: {
    "unused-imports/no-unused-imports": "error",
    "import/order": [
      "error",
      {
        groups: [
          "builtin",
          "external",
          "parent",
          "sibling",
          "index",
          "object",
          "type",
        ],
        "newlines-between": "always",
      },
    ],
    "@typescript-eslint/consistent-type-imports": [
      "error",
      { prefer: "type-imports" },
    ],
    "no-console": "warn",
    "no-cond-assign": "error",
    "no-constant-condition": "warn",
    "no-dupe-args": "warn",
    "no-dupe-keys": "error",
    "no-duplicate-case": "error",
    "no-extra-boolean-cast": "error",
    "no-extra-parens": "warn",
    "no-extra-semi": "warn",
    "no-ex-assign": "error",
    "no-func-assign": "error",
    "no-inner-declarations": "warn",
    "no-invalid-regexp": "error",
    "no-irregular-whitespace": "error",
    "no-unexpected-multiline": "error",
    "no-unreachable": "error",
    "no-unsafe-negation": "error",
    "use-isnan": "error",
    "block-scoped-var": "error",
    "default-case": "error",
    "dot-notation": "error",
    eqeqeq: "error",
    "no-eval": "warn",
    "no-global-assign": "error",
    "no-implicit-globals": "error",
    "no-implied-eval": "warn",
    "no-magic-numbers": "warn",
    "no-multi-spaces": "warn",
    "no-param-reassign": "error",
    "no-redeclare": "warn",
    "no-return-assign": "error",
    "no-return-await": "warn",
    "no-self-assign": "error",
    "no-self-compare": "error",
    "no-throw-literal": "error",
    "no-unused-expressions": "warn",
    "no-useless-concat": "warn",
    "no-useless-escape": "warn",
    "no-useless-return": "warn",
    "no-void": "error",
    yoda: "error",
    semi: "error",
    "init-declarations": "error",
    "@typescript-eslint/no-unused-vars": [
      "error",
      {
        argsIgnorePattern: "^_",
        varsIgnorePattern: "^_",
        caughtErrorsIgnorePattern: "^_",
        destructuredArrayIgnorePattern: "^_",
      },
    ],
    "no-use-before-define": "error",
    camelcase: "warn",
    "no-const-assign": "error",
    "no-duplicate-imports": "error",
    "no-var": "error",
    "prefer-arrow-callback": "error",
    "prefer-const": "error",
    "prefer-template": "error",
    "import/no-unresolved": "off",
    "@typescript-eslint/no-misused-promises": "warn",
  },
};
