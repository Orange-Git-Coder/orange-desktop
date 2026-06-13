import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import Pages from "vite-plugin-pages";
import { fileURLToPath, URL } from "node:url";

const host = Deno.env.get("TAURI_DEV_HOST");

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    Pages({
      dirs: "src/pages",
      extensions: ["vue"],
      extendRoute(route) {
        // 首页默认不使用 layout
        if (route.path === "/") {
          return { ...route, meta: { ...route.meta, layout: "default" } }
        }
        return route
      },
    }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));