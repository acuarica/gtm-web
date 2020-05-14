import App from './App.svelte'
import { WebService } from './web'
import { WorkdirStatusList, Commit, CommitsFilter } from '../../@notes'

const web = new WebService()

export default [{
  component: App,
  name: 'App with web service',
  props: {
    fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => web.fetchCommits(filter),
    fetchProjectList: (): Promise<string[]> => web.fetchProjectList(),
    fetchWorkdirStatus: (): Promise<WorkdirStatusList> => web.fetchWorkdirStatus(),
  }
}]