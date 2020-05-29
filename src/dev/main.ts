import App from '../app/App.svelte'
import Settings from '../app/Settings.svelte';
import { Commit, CommitsFilter, WorkdirStatusList } from '@gtm/notes';
import { WebService } from '../app/web';

console.debug('Creating main app with web service')

const service = new WebService();

new App({
  target: document.body,
  props: {
    fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => service.fetchCommits(filter),
    fetchProjectList: (): Promise<string[]> => service.fetchProjectList(),
    fetchWorkdirStatus: (): Promise<WorkdirStatusList> => service.fetchWorkdirStatus(),
    settingsView: Settings,
    settingsViewProps: { versions: {} },
  },
});
