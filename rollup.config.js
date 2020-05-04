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

import { startServe } from './make'

const production = !process.env.ROLLUP_WATCH;

export default {
  input: 'src/dev/main.js',
  output: {
    dir: 'dist-dev',
    sourcemap: true,
    format: 'iife',
    name: 'app',
  },
  plugins: [
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
    resolve({
      browser: true,
      // dedupe: ['svelte']
    }),
    commonjs({
      exclude: 'node_modules/moment/**/*',
      sourceMap: false
    }),
    json(),
    url({
      include: ['svg', 'png', 'woff', 'woff2', 'eot', 'ttf'].map(e => '**/*.' + e),
      limit: Infinity,
    }),
    html2({
      template: 'src/dev/index.html',
    }),
    postcss({
      modules: true,
      extract: 'assets/main.css',
      plugins: [
        require("postcss-import")(),
        require("tailwindcss"),

        // const purgecss = require("@fullhuman/postcss-purgecss");
        // require("autoprefixer"),
        // Only purge css on production
        // production &&
        // purgecss({
        //   content: ["./**/*.html", "./**/*.svelte"],
        //   defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || []
        // })

      ]
    }),
    copy({
      targets: [
        { src: "assets/gtm-logo.png", dest: "dist-dev/assets" },
        { src: "assets/css/*", dest: "dist-dev/assets/css" },
        { src: "assets/webfonts/*", dest: "dist-dev/assets/webfonts" },
      ],
    }),
    !production && serve32('dist-dev', 9090),
    !production && livereload('dist-dev'),
    production && terser(),
    progress({
      // clearLine: false // default: true
    })
  ],
  watch: {
    clearScreen: false
  }
};

function serve32(dir, port) {
  console.log(dir)
  let started = false;

  return {
    writeBundle() {
      if (!started) {
        started = true;
        startServe(dir, port)

        // require('child_process').spawn('npm', ['run', 'start', '--', '--dev'], {
        //   stdio: ['ignore', 'inherit', 'inherit'],
        //   shell: true
        // });
      }
    }
  };
}




