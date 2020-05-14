import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import terser from 'rollup-plugin-terser';
import copy from 'rollup-plugin-copy'
import postcss from 'rollup-plugin-postcss'
import image from '@rollup/plugin-image';
import purgecss from "@fullhuman/postcss-purgecss";
import html from '@open-wc/rollup-plugin-html';
import tailwindcss from 'tailwindcss'

const production = !process.env.ROLLUP_WATCH;

const plugins = (extract) => [
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
    extract: extract,
    plugins: [
      tailwindcss('./tailwind.config.cjs'),
      production && purgecss({
        content: ["./**/*.html", "./**/*.svelte"],
        defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || []
      })
    ]
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
      ...plugins(false),
      ...['dev/index', 'app/test'].map(f => html({ inputPath: `src/${f}.html` })),
    ],
    watch: {
      clearScreen: false
    }
  },

  gtm: {
    input: 'src/app/gtm.js',
    output: {
      dir: 'dist/gtm',
      format: 'iife',
      name: 'gtm',
    },
    plugins: [
      ...plugins(false),
      production && terser.terser(),
    ]
  },

  demo: {
    input: 'src/demo/index.html',
    output: {
      dir: 'dist/demo',
      format: 'iife',
      name: 'app',
    },
    plugins: [
      ...plugins(false),
      production && terser.terser(),
      html(),
      copy({
        targets: [
          { src: 'assets/data/', dest: 'dist/demo' },
        ],
      }),
    ]
  },

}

export default Object.values(configs)