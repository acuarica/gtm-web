#!/usr/bin/env node

import chalk from 'chalk';
import { spawn } from 'child_process'
import sirv from 'sirv';
import { GitService } from '@gtm/git';
import polka from 'polka';
import send from '@polka/send-type';
import * as rollup from 'rollup';

const DIST = 'dist'
const PORT = 9090

const ui = (() => {
  const ind = chalk.gray(`gtm Make `)
  let prefix = ind

  return {
    log: (text) => {
      process.stdout.write(`${prefix}${text}`)
      prefix = ""
    },

    logln: (text) => {
      process.stdout.write(`${prefix}${text}\n`)
      prefix = ind
    }
  }
})()

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

async function rollupWatch(opts) {
  ui.logln(chalk.cyanBright(`Starting rollup, options: ${JSON.stringify(opts)}`))
  process.env.ROLLUP_WATCH = true
  const { configs } = await import('./rollup.config.js')
  const watcher = rollup.watch(configs.dev);
  watcher.on('event', event => {
    ui.logln(chalk.magenta(event.code))
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
        // watcher.close()
      }
    }
  });
}

const commands = {

  dev: {
    desc: `Starts ${chalk.bold('tsc')} in watch mode, then runs rollup`,
    fn: () => {
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
  },

  test: {
    desc: `Starts tests ${chalk.bold('tsc')} in watch mode, then runs tests`,
    fn: () => {
      const mocha = async () => {
        spawnAnsi('yarn', ['test'], {
          stdio: ['ignore', 'inherit', 'inherit']
        });
      }
      let started = false
      tsWatch(() => {
        if (!started) {
          mocha()
        }
      })
    }
  },

  serve: {
    desc: "Starts local http server dist folder",
    fn: () => {
      const assets = sirv(DIST, {
        maxAge: 31536000, // 1Y
        immutable: true,
        dev: true,
      });

      const service = new GitService(args => spawn('yarn', ['--silent', 'gtm', ...args]))
      polka()
        .use(assets)
        .get('/data/commits', async (req, res) => {
          ui.logln(`Request: ${req.path}${req.search}`)
          const range = {
            start: req.query.from,
            end: req.query.to
          };
          if (range.start && range.end) {
            const data = await service.fetchCommits(range)
            send(res, 200, data);
          } else {
            console.warn("Argument to or from not defined:", range)
          }
        })
        .get('/data/projects', async (req, res) => {
          ui.logln(`Request projects: ${req.path}`)
          const data = await service.fetchProjectList()
          send(res, 200, data);
        })
        .get('/data/status', async (req, res) => {
          ui.logln(`Request workdir status: ${req.path}`)
          const data = await service.fetchWorkdirStatus()
          send(res, 200, data);
        })
        .listen(PORT, err => {
          if (err) throw err;
          ui.logln(`✨ Ready on localhost:${PORT}~ 🚀 !`);
        });
    }
  },
}

async function main(argv) {
  const usage = () => {
    ui.logln(chalk.bold('Available commands:'))
    for (const cmd in commands) {
      const pad = " ".repeat(12 - cmd.length)
      ui.logln(chalk(`    ${cmd}${pad} ${commands[cmd].desc}`))
    }
    process.exit(1)
  }

  if (argv.length <= 2) {
    ui.logln(chalk.red.bold('No command provided for make.mjs.'))
    usage()
  }

  argv = argv.slice(2)
  for (const arg of argv) {
    const cmd = commands[arg]
    if (!cmd) {
      ui.logln(chalk.red.bold(`Unrecognized command '${arg}'.`))
      usage()
    }
  }

  // const cfgs = []
  for (const arg of argv) {
    ui.log(chalk.blue.bold(`Running '${arg}' command ... `))
    const cmd = commands[arg]
    // cfgs.push(configs[argv[arg]])
    cmd.fn()
  }
}

main(process.argv)