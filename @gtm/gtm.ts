import { ChildProcessWithoutNullStreams } from 'child_process'
import { Commit, WorkdirStatusList, GtmService, CommitsFilter, GtmErr, FileNote } from '@gtm/notes'
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
    throw new GtmErr(outBuf, await exitCode ?? undefined)
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

export function parseFileEntry(entry: string): FileNote | null {
  const parts = entry.split(',')
  if (parts.length < 3) return null
  const [sourceFile, totalTimeSpent] = parts[0].split(':')
  if (!sourceFile) return null
  if (!Number(totalTimeSpent)) return null
  const status = parts[parts.length - 1]
  if (!['m', 'r', 'd'].includes(status)) return null

  const fileNote = new FileNote(
    sourceFile,
    Number(totalTimeSpent),
    {},
    status
  )
  for (let i = 1; i < parts.length - 1; i++) {
    const [timeStamp, timeSpent] = parts[i].split(':')
    if (!Number(timeSpent)) return null
    fileNote.Timeline[timeStamp] = Number(timeSpent)
  }
  return fileNote
}

export function gethistory(): Promise<unknown> {
  return Promise.resolve([])
  // eslint-disable-next-line no-async-promise-executor
  // return new Promise(async (resolve: any) => {
  //   const repo = await nodegit.Repository.open('/Users/luigi/work/#archive/vscode-lgtm-ql')
  //   const firstCommitOnMaster = await repo.getMasterCommit()
  //   const history = firstCommitOnMaster.history();
  //   history.on('commit', async function (commit) {
  //     console.log('commit ' + commit.sha());
  //     console.log('Author:', commit.author().name() +
  //       ' <' + commit.author().email() + '>');
  //     console.log('Date:', commit.date());
  //     console.log('\n    ' + commit.message());

  //     try {
  //       const note = await nodegit.Note.read(repo, 'refs/notes/gtm-data', commit.id())
  //       console.log(note.message())
  //     } catch (e) {
  //       // return Promise.reject()

  //     }
  //   });
  //   history.on('end', () => {
  //     resolve()
  //   })

  //   history.start();
  // })
}