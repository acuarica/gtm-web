
import { fetchCommits, fetchProjectList, fetchWorkdirStatus } from '../../@git/src/git.js'

import '../../main.pcss'

import App from '../app/App.svelte'
import Settings from './Settings.svelte'
import { Commit, WorkdirStatusList } from '@gtm/notes';

window.addEventListener('DOMContentLoaded', async () => {

  console.info('Creating app with gtm/git service')
  console.log('@preload', document.body)
  new App({
    target: document.body,
    props: {
      fetchCommits: async (range: { start: string; end: string }): Promise<Commit[]> => {
        return fetchCommits(range)
      },
      fetchProjectList: async (): Promise<string[]> => {
        return (await fetchProjectList()).map(p => p.substring(p.lastIndexOf('/') + 1))
      },
      fetchWorkdirStatus: async (): Promise<WorkdirStatusList> => {
        return fetchWorkdirStatus()
      },
      settingsView: Settings,
    },
  })

});