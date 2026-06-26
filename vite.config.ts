import { defineConfig } from "vite";
import path from "path";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  base: "./",
  server: {
    port: 6125,
    host: "0.0.0.0",
  },
  build: {
    outDir: "build",
    assetsDir: "media",
    sourcemap: true,
    minify: "terser",
    target: "es2020",
    cssCodeSplit: true,
    chunkSizeWarningLimit: 500,
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes("node_modules")) {
            const pkgName = id.match(
              /node_modules\/((@[^\/]+\/[^\/]+)|[^\/]+)/,
            )?.[1];
            if (pkgName) return `vendor-${pkgName.replace("@", "")}`;
          }
        },
      },
    },
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "@utils": path.resolve(__dirname, "./src/utils"),
      "@components": path.resolve(__dirname, "./src/components"),
      "@pages": path.resolve(__dirname, "./src/pages"),
      "@styles": path.resolve(__dirname, "./src/styles"),
    },
  },
  plugins: [vue({
    template:{
      compilerOptions:{
        isCustomElement: (tag) => ['lc', 'sc', 'imt'].includes(tag)
      }
    }
  })],
  envPrefix: "GS",
});
