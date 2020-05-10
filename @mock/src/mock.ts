import fs from 'fs'
import { GtmService, CommitsFilter, Commit, WorkdirStatusList, GtmErr } from '@gtm/notes';
import { parseDate, parseWhen } from '@gtm/format';

const [commits, projects, workdir] = ['commits', 'projects', 'workdir']
  .map(name => {
    const bytes = fs.readFileSync(`@mock/data/${name}.json`);
    return JSON.parse(bytes.toString());
  })

export class MockService implements GtmService {

  fetchCommits(filter: CommitsFilter): Promise<Commit[]> {
    const start = parseDate(filter.start)
    const end = parseDate(filter.end)

    if (!start) throw new GtmErr('Invalid start date')
    if (!end) throw new GtmErr('Invalid end date')

    const result = []

    for (const commit of commits) {
      const when = parseWhen((commit as Commit).When)
      if (start.isBefore(when) && end.isAfter(when)) {
        result.push(commit)
      }
    }

    return Promise.resolve(result)
  }

  fetchProjectList(): Promise<string[]> {
    return projects
  }

  fetchWorkdirStatus(): Promise<WorkdirStatusList> {
    return workdir
  }

}

export { commits, projects, workdir }