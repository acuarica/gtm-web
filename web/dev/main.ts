import App from '../app/App.svelte'
import Settings from '../app/Settings.svelte';
import { Commit, CommitsFilter, WorkdirStatusList } from '@gtm/notes';
import { WebService } from '../app/web';

export default (host?: string): void => {

  console.debug(`Creating main app with web service on ${host}`)

  const service = new WebService(host);

  new App({
    target: document.body,
    props: {
      fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => service.fetchCommits(filter),
      fetchProjectList: (): Promise<string[]> => service.fetchProjectList(),
      fetchWorkdirStatus: (): Promise<WorkdirStatusList> => service.fetchWorkdirStatus(),
      settingsView: Settings,
      settingsViewProps: { versions: {} },
    },
  })

}