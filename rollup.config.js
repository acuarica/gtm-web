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

  test: {
    input: '@app/test.html',
    output: {
      dir: 'dist/test',
      sourcemap: true,
      format: 'es',
      plugins: [
        !production && livereload('dist/test'),
      ]
    },
    preserveModules: true,
    plugins: [
      ...plugins(false),
      html(),
    ],
    watch: {
      clearScreen: false
    }
  },

  web: {
    input: '@web/index.html',
    output: {
      dir: 'dist/web',
      format: 'iife',
      name: 'app',
    },
    plugins: [
      ...plugins(false),
        !production && livereload('dist/web'),
      production && terser.terser(),
      html(),
    ]
  },

  electron: {
    input: ['main', 'preload'].map(f => `@electron/${f}.js`),
    output: {
      dir: 'dist/electron',
      sourcemap: true,
      format: 'cjs',
    },
    plugins: [
      ...plugins(true),
      copy({
        targets: [
          { src: '@electron/index.html', dest: 'dist/electron' },
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

}

export default Object.values(configs)