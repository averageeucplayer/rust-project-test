import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import terser from '@rollup/plugin-terser';
import { sveltePreprocess } from "svelte-preprocess";
import typescript from "@rollup/plugin-typescript";
import json from '@rollup/plugin-json';

const { default: { name: pkg_name, module, main }} = await import('./package.json', { with: { type: "json" }, });
const development = process.env.NODE_ENV == "development";
// const production = !process.env.ROLLUP_WATCH

const name = pkg_name
	.replace(/^(@\S+\/)?(svelte-)?(\S+)/, '$3')
	.replace(/^\w/, m => m.toUpperCase())
	.replace(/-\w/g, m => m[1].toUpperCase());

export default {
	input: 'src/index.ts',
	output: [
        {
            sourcemap: development,
            format: 'cjs',
            file: main,
        },
        {
            sourcemap: development,
            format: 'esm',
            file:  module,
        }
	],
	plugins: [
		svelte({
            preprocess: sveltePreprocess({ sourceMap: true }),
            compilerOptions: {
                dev: development
            },
            emitCss: true,
        }),
        resolve({
            browser: true,
            dedupe: ['svelte']
        }),
        terser(),
        json(),
        typescript({
            sourceMap: true,
            declaration: true,
            outDir: "dist",
            exclude: ['**/*.test.ts', '**/*.spec.ts', '**/__tests__/**']
        }),
	]
};