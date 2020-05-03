import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import json from '@rollup/plugin-json';
import typescript from '@rollup/plugin-typescript';

export default [
	{
		input: ['src/desktop/main.ts', 'src/desktop/preload.ts'],
		output: {
			dir: 'dist',
			format: 'iife',
			sourcemap: true
		},
		plugins: [
			resolve(),
			svelte({
				css: css => {
					css.write('static/svelte.css')
				},
			}),
			commonjs(),
			json(),
			typescript({
				typescript: require('typescript')
			})
		],
		experimentalCodeSplitting: true,
		experimentalDynamicImport: true,
		external: [
			'electron',
			'child_process',
			'fs',
			'path',
			'url',
			'module',
			'os'
		]
	}
];