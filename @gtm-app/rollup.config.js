import resolve from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
const svelteOptions = require("./svelte.config");
const html = require('@rollup/plugin-html');
import serve from 'rollup-plugin-serve'
import livereload from 'rollup-plugin-livereload'
import json from '@rollup/plugin-json';
import postcss from 'rollup-plugin-postcss'
import copy from "rollup-plugin-copy-assets";

const production = !process.env.ROLLUP_WATCH;

export default {
  input: './src/index.ts',
  output: {
    dir: 'dist/rollup',
    format: 'iife',
    sourcemap: true,
  },
  plugins: [
    // copy({
    //   assets: [
    //     "src/assets",
    //   ],
    // }),

    json(), // only in dev

    postcss({
      extract: true
    }),

    svelte({
      ...svelteOptions,

      // extensions: [".svelte"],
      // preprocess: sveltePreprocessor(),

    }),
    typescript(
      // {
      //   allowSyntheticDefaultImports: true,
      //   include: 'src'
      // }
    ),
    // commonjs({ include: "node_modules/**", extensions: [".js", ".ts"] }),
    commonjs({ sourceMap: false }),
    // commonjs(),
    resolve({
      browser: true,
      dedupe: importee =>
        importee === "svelte" || importee.startsWith("svelte/")
    }),
    html(),
    serve({
      port: 1234,
      contentBase: 'dist'
    }),
    !production && livereload('dist'),
  ]
};