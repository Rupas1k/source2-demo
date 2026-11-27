import js from "@eslint/js";
import { defineConfig } from "eslint/config";
import prettier from "eslint-config-prettier";
import tseslint from "typescript-eslint";

export default defineConfig(
  {
    ignores: [
      "dist/**",
      "node_modules/**",
      "src/generated/**",
      "wasm/target/**",
    ],
  },
  { files: ["**/*.{js,mjs,cjs}"], ...js.configs.recommended },
  ...tseslint.configs.recommended.map((config) => ({
    ...config,
    files: ["**/*.{ts,tsx}"],
  })),
  prettier,
);
