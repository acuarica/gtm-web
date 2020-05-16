import { ChildProcessWithoutNullStreams } from 'child_process'
import { Commit, WorkdirStatusList, GtmService, CommitsFilter, GtmErr } from '@gtm/notes'
import { parseDate } from '@gtm/notes'

export type GtmSpawn = (args: string[]) => ChildProcessWithoutNullStreams

async function runGtm<T>(gtm: GtmSpawn, args: string[]): Promise<T> {
  const child = gtm(args);

  const exitCode = new Promise<number | null>(resolve => {
    child.on('exit', code => {
      resolve(code)
    });
  });

  let outBuf = ''
  for await (const data of child.stdout) {
    outBuf += data
  }

  let errBuf = ''
  for await (const data of child.stderr) {
    errBuf += data
  }

  if (await exitCode === 0) {
    try {
      const json = JSON.parse(outBuf)
      return json
    } catch (err) {
      throw new GtmErr(outBuf + errBuf, 0)
    }
  } else {
    throw new GtmErr(outBuf + errBuf, await exitCode ?? undefined)
  }
}

export class GitService implements GtmService {

  constructor(readonly gtm: GtmSpawn) { }

  fetchCommits(filter: CommitsFilter): Promise<Commit[]> {
    const start = parseDate(filter.start)
    if (!start) throw new GtmErr(`Invalid start date: ${start}`)

    const end = parseDate(filter.end)
    if (!end) throw new GtmErr(`Invalid end date: ${end}`)

    const args = [
      'commits',
      `--from-date=${start.format('YYYY-MM-DD')}`,
      `--to-date=${end.add(1, 'day').format('YYYY-MM-DD')}`,
    ]

    return runGtm(this.gtm, args)
  }

  fetchProjectList(): Promise<string[]> {
    const args = ['projects']
    return runGtm(this.gtm, args)
  }

  fetchWorkdirStatus(): Promise<WorkdirStatusList> {
    const args = ['status']
    return runGtm(this.gtm, args)
  }

}
