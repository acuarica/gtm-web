import { spawn } from 'child_process'

export async function fetchCommits(range) {
  const args = [
    '-data=commits',
    `-from-date=${range.start}`,
    `-to-date=${range.end}`
  ]
  return rungtm(args)
}

export async function fetchProjectList() {
  const args = ['-data=projects']
  return rungtm(args)
}

export async function fetchWorkdirStatus() {
  const args = ['-data=status']
  return rungtm(args)
}

async function rungtm(args) {
  // const child = spawn('/usr/local/bin/gtm', ['report']);
  const gtmexec = '/Users/luigi/work/#forks/gtm/bin/gtm'
  args = ['export', ...args]
  const child = spawn(gtmexec, args);

  child.on('exit', code => {
    console.log(`Exit code is: ${code}, ${args}`);
  });

  let buf = ""
  for await (const data of child.stdout) {
    buf += data
  };

  return JSON.parse(buf)
}