import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import json from '@rollup/plugin-json';
import { terser } from 'rollup-plugin-terser';
import html2 from 'rollup-plugin-html2'
import copy from 'rollup-plugin-copy'
import url from '@rollup/plugin-url';
import postcss from 'rollup-plugin-postcss'
import progress from 'rollup-plugin-progress';
import image from '@rollup/plugin-image';
import sizes from 'rollup-plugin-sizes';
import purgecss from "@fullhuman/postcss-purgecss";

const production = process.env.ROLLUP_WATCH;

const plugins = (dir) => [
  svelte({
    dev: !production,
    // css: css => {
    //   css.write('dist-dev/gtm-svelte.css');
    // }
  }),

  // If you have external dependencies installed from
  // npm, you'll most likely need these plugins. In
  // some cases you'll need additional configuration -
  // consult the documentation for details:
  // https://github.com/rollup/plugins/tree/master/packages/commonjs
  commonjs({
    exclude: 'node_modules/moment/**/*',
    sourceMap: false
  }),
  json(),
  image(),
  url({
    include: ['svg', 'png', 'woff', 'woff2', 'eot', 'ttf'].map(e => '**/*.' + e),
    limit: Infinity,
  }),

  postcss({
    modules: true,
    extract: true,//'assets/main.css',
    plugins: [
      require("postcss-import")(),
      require("tailwindcss"),
      // require("autoprefixer"),
      // Only purge css on production
      production && purgecss({
        content: ["./**/*.html", "./**/*.svelte"],
        defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || []
      })

    ]
  }),
  copy({
    targets: [
      // { src: "assets/gtm-logo.png", dest: `${dir}/assets` },
      // { src: "assets/css/*", dest: `${dir}/assets/css` },
      // { src: "assets/webfonts/*", dest: `${dir}/assets/webfonts` },
      // { src: "mock/data-*.json", dest: `${dir}/data` },
    ],
  }),
  production && terser(),
  progress({
    // clearLine: false // default: true
  }),
  // sizes(),
]

export default [{
  // input: 'src/dev/main.js',
  input: ['App', 'Navbar'].map(f=>`src/app/${f}.svelte`),
  output: {
    dir: 'dist-dev',
    // sourcemap: true,
    format: 'iife',
    name: 'app',
  },
  plugins: [

    resolve({
      browser: true,
      // dedupe: ['svelte']
    }),
    html2({
      template: 'src/dev/index.html',
      inject: true,
    }),
    ...plugins('dist-dev'),
    // !production && livereload('dist-dev'),
  ],
  watch: {
    clearScreen: false
  }
}, {
  input: ['main', 'preload'].map(f => `src/desktop/${f}.js`),
  output: {
    dir: 'dist-electron',
    // sourcemap: true,
    format: 'cjs',
    // name: 'app',
  },
  plugins: [
    resolve({
      // browser: false,
      // dedupe: ['svelte']
    }),
    ...plugins('dist-electron'),

    copy({
      targets: [
        { src: 'src/desktop/index.html', dest: 'dist-electron' },
      ],
    }),
  ],
  external: ['electron', 'child_process', 'fs', 'path', 'url', 'module', 'os'],
  watch: {
    clearScreen: false
  }
}][0];
