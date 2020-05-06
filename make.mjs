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
import json from '@rollup/plugin-json';
import terser from 'rollup-plugin-terser';
import html2 from 'rollup-plugin-html2'
import copy from 'rollup-plugin-copy'
import url from '@rollup/plugin-url';
import postcss from 'rollup-plugin-postcss'
import progress from 'rollup-plugin-progress';
import image from '@rollup/plugin-image';
import sizes from 'rollup-plugin-sizes';
import purgecss from "@fullhuman/postcss-purgecss";

import pcssimport from "postcss-import"
import tw from "tailwindcss"

import yargs from 'yargs'

const ind = chalk.gray(`gtm Make`)
const log = (buffer) => process.stdout.write(buffer);
const logln = (buffer) => process.stdout.write(`${ind} - ${buffer}\n`);

const production = !process.argv.includes('tsc')

const rollupConfig = {
  input: 'src/dev/main.js',
  output: {
    dir: 'dist-dev',
    sourcemap: !production,
    format: 'iife',
    name: 'app',
  },
  plugins: [
    svelte({
      dev: !production,
    }),
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
      // extract: 'assets/main.css',
      plugins: [
        pcssimport(),
        tw,
        // require("autoprefixer"),
        production && purgecss({
          content: ["./**/*.html", "./**/*.svelte"],
          defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || []
        })
      ]
    }),
    html2({
      template: 'src/dev/index.html',
      // inject: true,
    }),
    !production && startServe('dist-dev', 9090),
    !production && livereload('dist-dev'),
    // production && terser(),
    progress({
      // clearLine: false // default: true
    }),
    production && sizes(),
  ]
}

function startServe(dir, port) {
  let started = false;

  return {
    writeBundle() {
      if (!started) {
        started = true;
        serveGtm(dir, port)
      }
    }
  };
}

function serveGtm(dir, port) {
  const assets = sirv(dir, {
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
    .listen(port, err => {
      if (err) throw err;
      logln(`✨ Ready on localhost:${port}~ 🚀 !`);
    });
}

export async function tsWatch() {
  const child = spawn('yarn', ['tsc', '--watch', '--preserveWatchOutput', '--noEmitOnError'], {
    env: {
      FORCE_COLOR: 1,
      ...process.env
    }
  });

  let rollupStarted = false
  for await (const data of child.stdout) {
    process.stdout.write(data)
    const line = Buffer.from(data, 'utf8').toString()
    const emitComplete = line.includes('Found 0 errors')
    if (emitComplete) {
      console.info("TypeScript emit code complete 🚀 !")
      if (!rollupStarted) {
        rollupStarted = true
        rollupWatch()
      }
    }
  }
}

async function rollupWatch() {
  // const inputOptions = {
  //   input: rollupConfig.input,
  //   plugins: rollupConfig.plugins,
  // }
  // const bundle = await rollup.rollup(inputOptions);

  const watcher = rollup.watch(rollupConfig);
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
    }
  });

  // console.log(bundle.watchFiles);
}

async function main() {
  const usage = () => {
    logln(chalk.bold('Available commands:'))
    for (const cmd in cmds) {
      logln(chalk(`    ${cmd}`))
    }
    logln('')
    process.exit(1)
  }

  logln(chalk.gray(`gtm Make`))

  const cmds = {
    tsc: tsWatch,
    rollup: rollupWatch,
    serve: serveGtm,
  }

  let argv = process.argv
  if (argv.length <= 2) {
    logln(chalk.red.bold('No command provided for make.mjs'))
    usage()
  }

  argv = argv.slice(2)

  for (const arg in argv) {
    const cmd = cmds[argv[arg]]
    if (!cmd) {
      logln(chalk.red.bold(`Command '${argv[arg]}' is undefined.`))
      usage()
      process.exit(1)
    }
  }

  for (const arg in argv) {
    logln(chalk.blue.bold(`Running '${argv[arg]}' ...`))
    const cmd = cmds[argv[arg]]
    cmd()
  }
}

main()