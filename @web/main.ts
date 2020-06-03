import App from '../@app/App.svelte'
import Settings from '../@app/Settings.svelte';
import { Commit, CommitsFilter, WorkdirStatusList } from '@gtm/notes';
import { AuthWebService } from '../@app/web';

console.debug('Creating main app with web service')

function getUrlParams(search: string): { [key: string]: string } {
  const hashes = search.slice(search.indexOf('?') + 1).split('&')
  const params: { [key: string]: string } = {}
  hashes.map(hash => {
    const [key, val] = hash.split('=')
    params[key] = decodeURIComponent(val)
  })
  return params
}

const params = getUrlParams(window.location.search)
const token = params['access_token']
console.debug(token)
if (token) {
  const service = new AuthWebService(token);

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

} else {

  console.warn('Not a valid token')
}