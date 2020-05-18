import App from './App.svelte'
import { WebService, DelayedService, FailureService } from './web'
import { WorkdirStatusList, Commit, CommitsFilter } from '@gtm/notes'

export default [
  { service: new WebService(), name: 'web service' },
  { service: new FailureService(), name: 'failure web service' },
  { service: new DelayedService(new WebService(), 3000), name: 'delayed web service' },
  { service: new DelayedService(new FailureService(), 4000), name: 'delayed failure web service' },
].map(t => {
  return {
    component: App,
    name: `with ${t.name}`,
    props: {
      fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => t.service.fetchCommits(filter),
      fetchProjectList: (): Promise<string[]> => t.service.fetchProjectList(),
      fetchWorkdirStatus: (): Promise<WorkdirStatusList> => t.service.fetchWorkdirStatus(),
    }
  }
})
