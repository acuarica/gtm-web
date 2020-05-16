import App from './App.svelte'
import { WebService, DelayedService, FailureService } from './web'
import { WorkdirStatusList, Commit, CommitsFilter } from '@gtm/notes'

const web = new WebService()
const delay = new DelayedService(web, 3000)
const fail = new FailureService(delay)

export default [{
  component: App,
  name: 'App with web service',
  props: {
    fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => web.fetchCommits(filter),
    fetchProjectList: (): Promise<string[]> => web.fetchProjectList(),
    fetchWorkdirStatus: (): Promise<WorkdirStatusList> => web.fetchWorkdirStatus(),
  }
}, {
  component: App,
  name: 'App using delayed web service',
  props: {
    fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => delay.fetchCommits(filter),
    fetchProjectList: (): Promise<string[]> => delay.fetchProjectList(),
    fetchWorkdirStatus: (): Promise<WorkdirStatusList> => delay.fetchWorkdirStatus(),
  }
}, {
  component: App,
  name: 'App using failure web service',
  props: {
    fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => fail.fetchCommits(filter),
    fetchProjectList: (): Promise<string[]> => fail.fetchProjectList(),
    fetchWorkdirStatus: (): Promise<WorkdirStatusList> => fail.fetchWorkdirStatus(),
  }
}]