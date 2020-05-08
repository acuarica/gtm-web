import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import terser from 'rollup-plugin-terser';
import copy from 'rollup-plugin-copy'
import postcss from 'rollup-plugin-postcss'
import progress from 'rollup-plugin-progress';
import image from '@rollup/plugin-image';
import sizes from 'rollup-plugin-sizes';
import purgecss from "@fullhuman/postcss-purgecss";
// const html = require('@rollup/plugin-html');
import html from '@open-wc/rollup-plugin-html';
import tailwindcss from 'tailwindcss'

const production = !process.env.ROLLUP_WATCH;

const plugins = [
  resolve({
    browser: true,
  }),
  commonjs({
    sourceMap: true
  }),
  image(),
  svelte({
    dev: !production,
  }),
  postcss({
    // modules: true,
    // extract: true,//'assets/main.css',
    plugins: [
      // require("postcss-import")(),
      tailwindcss('./tailwind.config.cjs'),
      // require("tailwindcss"),
      // require("autoprefixer"),
      // Only purge css on production
      purgecss({
        content: ["./**/*.html", "./**/*.svelte"],
        defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || []
      })
    ]
  }),
  progress({
    clearLine: true,
  }),
]

export const configs = {

  dev: {
    output: {
      dir: 'dist/dev',
      sourcemap: true,
      format: 'es',
      plugins: [

      !production && livereload('dist/dev'),
      ]
    },
    preserveModules: true,
    plugins: [
      ...plugins,
      ...['dev/index', 'test/test'].map(f => html({ inputPath: `src/${f}.html` })),
    ],
    watch: {
      clearScreen: false
    }
  },

  gtm: {
    input: 'src/dev/index.html',
    output: {
      dir: 'dist/gtm',
      format: 'iife',
      name: 'app',
    },
    plugins: [
      ...plugins,
      production && terser.terser(),
      html(),
    ]
  },

  electron: {
    input: ['main', 'preload'].map(f => `src/desktop/${f}.js`),
    output: {
      dir: 'dist/electron',
      sourcemap: true,
      format: 'cjs',
    },
    plugins: [
      ...plugins,
      copy({
        targets: [
          { src: 'src/desktop/index.html', dest: 'dist/electron' },
        ],
      }),
    ],
    external: ['electron', 'child_process', 'fs', 'path', 'url', 'module', 'os'],
    watch: {
      clearScreen: false
    }
  }

}

export default Object.values(configs)