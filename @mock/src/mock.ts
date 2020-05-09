import fs from 'fs'
import { GtmService, CommitsFilter, Commit, WorkdirStatusList } from '@gtm/notes';

const [commits, projects, workdir] = ['commits', 'projects', 'workdir']
  .map(name => {
    const bytes = fs.readFileSync(`@mock/data/${name}.json`);
    return JSON.parse(bytes.toString());
  })

export class MockService implements GtmService {

  fetchCommits(filter: CommitsFilter): Promise<Commit[]> {
    // for (const commit of commits) {
    //   if (commit.When)
    // }
    console.log(filter)
    return commits
  }

  fetchProjectList(): Promise<string[]> {
    return projects
  }

  fetchWorkdirStatus(): Promise<WorkdirStatusList> {
    return workdir
  }

}

export { commits, projects, workdir }