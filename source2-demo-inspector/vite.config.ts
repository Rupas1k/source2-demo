import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

export default defineConfig({
  base: "./",
  plugins: [
    react(),
    tailwindcss(),
    {
      name: "inspector-sample-page",
      apply: "build",
      generateBundle: {
        order: "post",
        handler(_, bundle) {
          const page = bundle["index.html"];
          if (page?.type !== "asset" || typeof page.source !== "string")
            this.error("Inspector HTML is missing from the build.");
          this.emitFile({
            type: "asset",
            fileName: "sample/index.html",
            source: page.source.replace(
              "<head>",
              '<head>\n    <base href="../">',
            ),
          });
        },
      },
    },
  ],
  worker: { format: "es" },
});
