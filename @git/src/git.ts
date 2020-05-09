import moment from 'moment'
import { spawn } from 'child_process'
import { Commit, WorkdirStatusList } from '@gtm/notes'

async function rungtm(args: string[]): Promise<ReturnType<JSON['parse']>> {
  const gtmexec = '/Users/luigi/work/#forks/gtm/bin/gtm'
  args = ['export', ...args]
  const child = spawn(gtmexec, args);

  const exitCode = new Promise<number | null>(resolve => {
    child.on('exit', code => {
      resolve(code)
    });
  });

  let buf = ''
  for await (const data of child.stdout) {
    buf += data
  }

  if (await exitCode === 0) {
    return JSON.parse(buf)
  } else {
    return { err: exitCode, message: buf }
  }
}

export async function fetchCommits(range: { start: string; end: string }): Promise<Commit[]> {
  const end = moment(range.end, 'YYYY-MM-DD').add(1, 'day')
  const args = [
    '-data=commits',
    `-from-date=${range.start}`,
    `-to-date=${end.format('YYYY-MM-DD')}`,
  ]
  return rungtm(args)
}

export async function fetchProjectList(): Promise<string[]> {
  const args = ['-data=projects']
  return rungtm(args)
}

export async function fetchWorkdirStatus(): Promise<WorkdirStatusList> {
  const args = ['-data=status']
  return rungtm(args)
}