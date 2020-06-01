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
      ...['test'].map(f => html({ inputPath: `@app/${f}.html` })),
    ],
    watch: {
      clearScreen: false
    }
  },

  appdesktop: {
    input: ['main', 'preload'].map(f => `src/desktop/${f}.js`),
    output: {
      dir: 'dist/gtm-dash',
      sourcemap: true,
      format: 'cjs',
    },
    plugins: [
      ...plugins(true),
      copy({
        targets: [
          { src: 'src/desktop/index.html', dest: 'dist/gtm-dash' },
          ...['', '.exe'].map(ext => {
            return { src: `gtmserv/target/${production ? 'release' : 'debug'}/gtmserv${ext}`, dest: 'dist/gtm-dash' }
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