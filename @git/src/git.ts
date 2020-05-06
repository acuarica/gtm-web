import moment from "moment"
import { spawn } from 'child_process'
import { Commit, FileNote } from '@gtm/notes'

async function rungtm(args: string[]): Promise<ReturnType<JSON['parse']>> {
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

export async function fetchCommits(range: { start: string; end: string }): Promise<Commit[]> {
  const end = moment(range.end, 'YYYY-MM-DD').add(1, 'day')
  const args = [
    '-data=commits',
    `-from-date=${range.start}`,
    `-to-date=${end.format('YYYY-MM-DD')}`
  ]
  return rungtm(args)
}

export async function fetchProjectList(): Promise<string[]> {
  const args = ['-data=projects']
  return rungtm(args)
}

export async function fetchWorkdirStatus(): Promise<{ [p: string]: { CommitNote: { Files: FileNote[] } } }> {
  const args = ['-data=status']
  return rungtm(args)
}