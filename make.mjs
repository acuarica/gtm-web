#!/usr/bin/env node

import chalk from 'chalk';
import { spawn } from 'child_process'
import sirv from 'sirv';
import { fetchCommits, fetchProjectList, fetchWorkdirStatus } from './src/git.js';
import polka from 'polka';
import send from '@polka/send-type';

const log = (buffer) => process.stdout.write(buffer);
const logln = (buffer) => process.stdout.write(buffer + '\n');

export function servegtm(dir, port) {
  const assets = sirv(dir, {
    maxAge: 31536000, // 1Y
    immutable: true,
    dev: true,
  });

  polka()
    .use(assets)
    .get('/data/commits', async (req, res) => {
      console.info(`Request: ${req.path}${req.search}`)
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
      console.info(`Request projects: ${req.path}`)
      const data = await fetchProjectList()
      send(res, 200, data);
    })
    .get('/data/status', async (req, res) => {
      console.info(`Request workdir status: ${req.path}`)
      const data = await fetchWorkdirStatus()
      send(res, 200, data);
    })
    .listen(port, err => {
      if (err) throw err;
      console.log(`✨ Ready on localhost:${port}~ 🚀 !`);
    });
}

export async function tswatch() {
  const child = spawn('yarn', ['tsc', '--watch', '--preserveWatchOutput', '--noEmitOnError'], {
    env: {
      FORCE_COLOR: 1,
      ...process.env
    }
  });

  for await (const data of child.stdout) {
    process.stdout.write(data)
    const line = Buffer.from(data, 'utf8').toString()
    const emitComplete = line.includes('Found 0 errors')
    if (emitComplete) {
      console.info("Initial compilation complete 🚀 !")
    }
  }
}

async function main(argv) {
  const cmds = {
    servegtm: servegtm,
    tswatch: tswatch,
  }
  logln(chalk.gray(`gtm Make`))
  const cmd = cmds[argv[0]]
  if (!cmd) {
    logln(chalk.red.bold(`Command '${argv[0]}' is undefined.`))
    process.exit(code)
  }
  cmd(...argv.slice(1))
}

if (process.argv.length > 2) {
  main(process.argv.slice(2))
}