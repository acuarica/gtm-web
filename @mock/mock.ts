// import fs from 'fs'
import { GtmService, CommitsFilter, Commit, WorkdirStatusList, GtmErr } from '@gtm/notes';
import { parseDate, parseWhen } from '@gtm/notes';

export type Loader = (name: string) => Promise<unknown>

export class MockService implements GtmService {

  private constructor(
    readonly commits: Commit[],
    readonly projects: string[],
    readonly workdir: WorkdirStatusList) {
  }

  static async create(loader: Loader): Promise<MockService> {
    const service = new MockService(
      await loader('commits') as Commit[],
      await loader('projects') as string[],
      await loader('workdir') as WorkdirStatusList)
    return service
  }

  fetchCommits(filter: CommitsFilter): Promise<Commit[]> {
    const start = parseDate(filter.start)
    const end = parseDate(filter.end)

    if (!start) throw new GtmErr('Invalid start date')
    if (!end) throw new GtmErr('Invalid end date')

    const result = []

    for (const commit of this.commits) {
      const when = parseWhen((commit as Commit).When)
      if (start.isBefore(when) && end.isAfter(when)) {
        result.push(commit)
      }
    }

    return Promise.resolve(result)
  }

  async fetchProjectList(): Promise<string[]> {
    return Promise.resolve(this.projects.map((p: string) => p.substring(p.lastIndexOf('/') + 1)))
  }

  async fetchWorkdirStatus(): Promise<WorkdirStatusList> {
    return Promise.resolve(this.workdir)
  }

}
