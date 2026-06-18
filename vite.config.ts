import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { resolve } from "node:path";

export default defineConfig({
    plugins: [vue()],

    resolve: {
        alias: {
            "@": resolve(__dirname, "src"),
        },
    },

    css: {
        preprocessorOptions: {
            scss: {
                additionalData: "",
            },
        },
    },

    server: {
        port: 1420,
        strictPort: true,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },

    clearScreen: false,
});