import resolve from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
const svelteOptions = require("./svelte.config");
const html = require('@rollup/plugin-html');
import serve from 'rollup-plugin-serve'
import livereload from 'rollup-plugin-livereload'
import json from '@rollup/plugin-json';

export default {
  input: './src/index.ts',
  output: {
    dir: 'dist',
    format: 'iife',
    sourcemap: true,
    globals: {
      // 'jquery': 'JQ',
    },
  },
  plugins: [
    json(), // only in dev
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
    commonjs({ include: "node_modules/**", extensions: [".js", ".ts"] }),
    // commonjs(),
    resolve({ browser: true ,
    
      dedupe: importee =>
        importee === "svelte" || importee.startsWith("svelte/")
    
    }),
    html(),
    serve({
      verbose: true,
      contentBase:'dist'}),
      livereload('dist'),



  ]
};