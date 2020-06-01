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

  app: {
    input: '@app/app.js',
    output: {
      dir: 'dist/app',
      format: 'iife',
      name: 'app',
    },
    plugins: [
      ...plugins(false),
      production && terser.terser(),
    ]
  },

  dev: {
    output: {
      dir: 'dist/dev',
      sourcemap: true,
      format: 'es',
      plugins: [
        !production && livereload('dist/test'),
      ]
    },
    preserveModules: true,
    plugins: [
      ...plugins(false),
      html({ inputPath: '@app/test.html' }),
    ],
    watch: {
      clearScreen: false
    }
  },

  electron: {
    input: ['main', 'preload'].map(f => `web/electron/${f}.js`),
    output: {
      dir: 'dist/electron',
      sourcemap: true,
      format: 'cjs',
    },
    plugins: [
      ...plugins(true),
      copy({
        targets: [
          { src: 'web/electron/index.html', dest: 'dist/electron' },
          ...['', '.exe'].map(ext => {
            return { src: `target/${production ? 'release' : 'debug'}/gtmcli${ext}`, dest: 'dist/electron' }
          })
        ],
      }),
    ],
    external: ['electron', 'child_process', 'fs', 'path', 'url', 'module', 'os'],
    watch: {
      clearScreen: false
    }
  },

  demo: {
    input: 'web/demo/index.html',
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