import '../../main.pcss'
import App from '../app/App.svelte'
import { MockService } from '@gtm/mock';
import { Commit, WorkdirStatusList } from '../../@notes';

async function fetchUrl<T>(name: string): Promise<T> {
  return await fetch(`data/${name}.json`).then(r => r.json())
}

console.debug('Creating app with demo service')

const service = MockService.create(fetchUrl)

new App({
  target: document.body,
  props: {
    fetchCommits: async (filter: { start: string; end: string }): Promise<Commit[]> => {
      return (await service).fetchCommits(filter)
    },
    fetchProjectList: async (): Promise<string[]> => {
      return (await service).fetchProjectList()
    },
    fetchWorkdirStatus: async (): Promise<WorkdirStatusList> => {
      return (await service).fetchWorkdirStatus()
    },
  },
});
