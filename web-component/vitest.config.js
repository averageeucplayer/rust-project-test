import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { sveltePreprocess } from 'svelte-preprocess';
import { svelteTesting } from '@testing-library/svelte/vite';

export default defineConfig(({ mode }) => ({
    plugins: [
        svelte({
            hot: !process.env.VITEST,
            preprocess: [sveltePreprocess({ typescript: true })]
        }),
        svelteTesting(),
    ],
    test: {
        globals: true,
        environment: 'jsdom',
        setupFiles: ["./setupTests.js"],
        coverage: {
            reporter: ['lcov'],
            include: [
                'src/**',
            ],
        }
    },
    resolve: {
        conditions: mode === 'test' ? ['browser'] : [],
    }
}));