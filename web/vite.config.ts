import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

// process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [sveltekit()],
	resolve: process.env.VITEST
		? {
				conditions: ['browser']
			}
		: undefined,
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ["./setupTests.js"],
    coverage: {
      reporter: ['lcov'],
      include: [
        'src/**', // Ensure source files are included
      ],
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
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));