#!/usr/bin/env node

import chalk from 'chalk';
import { spawn } from 'child_process'
import sirv from 'sirv';
import { fetchCommits, fetchProjectList, fetchWorkdirStatus } from '@gtm/git';
import polka from 'polka';
import send from '@polka/send-type';
import * as rollup from 'rollup';

import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import terser from 'rollup-plugin-terser';
import html2 from 'rollup-plugin-html2'
import postcss from 'rollup-plugin-postcss'
import progress from 'rollup-plugin-progress';
import image from '@rollup/plugin-image';
import sizes from 'rollup-plugin-sizes';
import purgecss from "@fullhuman/postcss-purgecss";

import pcssimport from "postcss-import"
import tw from "tailwindcss"

const DIST = 'dist-dev'
const PORT = 9090

const ind = chalk.gray(`gtm Make`)
const logln = (buffer) => process.stdout.write(`${ind} ${buffer}\n`);

function rollupConfig(opts) {
  opts = opts || {}
  opts.production = opts.production || !opts.watch

  return {
    input: 'src/dev/main.js',
    output: {
      dir: 'dist-dev',
      sourcemap: !opts.production,
      format: 'iife',
      name: 'app',
    },
    plugins: [
      svelte({
        dev: !opts.production,
        ...(!opts.production ? {} : {
          // css: css => {
          //   css.write('dist-dev/gtm-svelte.css')
          // }
        })
      }
      ),
      resolve({
        // browser: true,
        // dedupe: ['svelte']
      }),
      commonjs({
        exclude: 'node_modules/moment/**/*',
        sourceMap: false
      }),
      image(),
      postcss({
        modules: true,
        // extract: !opts.production,
        plugins: [
          pcssimport(),
          tw,
          // require("autoprefixer"),
          opts.production && purgecss({
            content: ["./**/*.html", "./**/*.svelte"],
            defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || []
          })
        ]
      }),
      html2({
        template: 'src/dev/index.html',
        // modules: true,
        // inject: true,
      }),
      progress({}),
      opts.watch && startServe(DIST, PORT),
      opts.watch && livereload(DIST),
      opts.production && terser.terser(),
      opts.production && sizes(),
    ]
  }
}

function startServe() {
  let started = false;

  return {
    writeBundle() {
      if (!started) {
        started = true;
        serveGtm()
      }
    }
  };
}

function serveGtm() {
  const assets = sirv(DIST, {
    maxAge: 31536000, // 1Y
    immutable: true,
    dev: true,
  });

  polka()
    .use(assets)
    .get('/data/commits', async (req, res) => {
      logln(`Request: ${req.path}${req.search}`)
      const range = {
        start: req.query.from,
        end: req.query.to
      };
      if (range.start && range.end) {
        const data = await fetchCommits(range)
        send(res, 200, data);
      } else {
        console.warn("Argument to or from not defined:", range)
      }
    })
    .get('/data/projects', async (req, res) => {
      logln(`Request projects: ${req.path}`)
      const data = await fetchProjectList()
      send(res, 200, data);
    })
    .get('/data/status', async (req, res) => {
      logln(`Request workdir status: ${req.path}`)
      const data = await fetchWorkdirStatus()
      send(res, 200, data);
    })
    .listen(PORT, err => {
      if (err) throw err;
      logln(`✨ Ready on localhost:${PORT}~ 🚀 !`);
    });
}

function spawnAnsi(command, args, opts) {
  return spawn(command, args, {
    env: {
      FORCE_COLOR: 1,
      ...process.env
    },
    ...opts
  });
}

async function tsWatch(func) {
  const child = spawnAnsi('yarn', ['tsc', '--watch', '--preserveWatchOutput', '--noEmitOnError']);

  for await (const data of child.stdout) {
    process.stdout.write(data)
    const line = Buffer.from(data, 'utf8').toString()
    const emitComplete = line.includes('Found 0 errors')
    if (emitComplete) {
      console.info("TypeScript emit code complete 🚀 !")
      if (func) {
        func({ watch: true })
      }
    }
  }
}

async function mocha() {
  spawnAnsi('yarn', ['mocha', '@*/test/*.js'], {
    stdio: ['ignore', 'inherit', 'inherit']
  });
}

async function rollupWatch(opts) {
  logln(chalk.cyanBright(`Starting rollup, options: ${JSON.stringify(opts)}`))
  const watcher = rollup.watch(rollupConfig(opts));
  watcher.on('event', event => {
    logln(chalk.magenta(event.code))
    // event.code can be one of:
    //   START        — the watcher is (re)starting
    //   BUNDLE_START — building an individual bundle
    //   BUNDLE_END   — finished building a bundle
    //   END          — finished building all bundles
    //   ERROR        — encountered an error while bundling
    if (event.code === 'ERROR') {
      console.log(event)
    } else if (event.code === 'END') {
      if (!opts || !opts.watch) {
        watcher.close()
      }
    }
  });
}

function dev() {
  let started = false
  tsWatch(opts => {
    if (!started) {
      started = true
      rollupWatch({
        production: false,
        ...opts,
      })
    }
  })
}

function devtest() {
  let started = false
  tsWatch(() => {
    if (!started) {
      mocha()
    }
  })
}

async function main() {
  const usage = () => {
    logln(chalk.bold('Available commands:'))
    for (const cmd in cmds) {
      logln(chalk(`    ${cmd}`))
    }
    process.exit(1)
  }

  const cmds = {
    tsc: tsWatch,
    rollup: rollupWatch,
    serve: serveGtm,
    dev: dev,
    devtest: devtest,
  }

  let argv = process.argv
  if (argv.length <= 2) {
    logln(chalk.red.bold('No command provided for make.mjs.'))
    usage()
  }

  argv = argv.slice(2)
  for (const arg in argv) {
    const cmd = cmds[argv[arg]]
    if (!cmd) {
      logln(chalk.red.bold(`Command '${argv[arg]}' is undefined.`))
      usage()
    }
  }

  for (const arg in argv) {
    logln(chalk.blue.bold(`Running '${argv[arg]}' command ...`))
    const cmd = cmds[argv[arg]]
    cmd()
  }
}

main()