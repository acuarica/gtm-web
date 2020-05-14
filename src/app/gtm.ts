import App from './App.svelte'
import { WebService } from './web';
import { CommitsFilter, WorkdirStatusList, Commit } from '@gtm/notes';

console.info('Loading gtm module')

export default (origin: string): void => {

  console.info('Creating main gtm app with web service, origin:', origin)

  const web = new WebService(origin)

  new App({
    target: document.body,
    props: {
      fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => web.fetchCommits(filter),
      fetchProjectList: (): Promise<string[]> => web.fetchProjectList(),
      fetchWorkdirStatus: (): Promise<WorkdirStatusList> => web.fetchWorkdirStatus(),
    },
  })

}