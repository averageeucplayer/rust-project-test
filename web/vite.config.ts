import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

const { host, NODE_ENV, VITEST } = process.env;

export default defineConfig(async () => ({
    plugins: [sveltekit()],
    resolve: {
    alias: [
            {
                find: "/web-lib/",
                replacement: "web-fake"
            }
        ],
        conditions: VITEST ? ['browser'] : undefined
    },
    test: {
        globals: true,
        environment: 'jsdom',
        setupFiles: ["./setupTests.js"],
        coverage: {
                reporter: ['lcov'],
                include: ['src/**'],
        }
    },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
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
        watch: {},
    },
}));
