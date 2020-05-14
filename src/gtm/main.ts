import '../../main.pcss'
import App from '../app/App.svelte'
import { WebService } from '../app/web';
import { CommitsFilter, WorkdirStatusList, Commit } from '../../@notes';

console.debug('Creating main gtm app with rust jjkj service')

const web = new WebService('http://localhost:8000')

export default new App({
  target: document.body,
  props: {
    fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => web.fetchCommits(filter),
    fetchProjectList: (): Promise<string[]> => web.fetchProjectList(),
    fetchWorkdirStatus: (): Promise<WorkdirStatusList> => web.fetchWorkdirStatus(),
  },
});