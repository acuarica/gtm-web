import { GitService } from '../../@git/src/git.js'
import App from '../app/App.svelte'
import Settings from './Settings.svelte'
import { Commit, WorkdirStatusList } from '@gtm/notes';
import { spawn } from 'child_process';

import '../../main.pcss'

window.addEventListener('DOMContentLoaded', async () => {

  // const service = new GitService((args: string[]) => spawn('gtm', args))
  const service = new GitService(args => spawn('yarn', ['--silent', 'gtm', ...args]))

  console.info('Creating app with gtm/git service')
  console.log('@preload', document.body)
  new App({
    target: document.body,
    props: {
      fetchCommits: async (range: { start: string; end: string }): Promise<Commit[]> => {
        return service.fetchCommits(range)
      },
      fetchProjectList: async (): Promise<string[]> => {
        const ps = await service.fetchProjectList()
        return ps.map(p => p.substring(p.lastIndexOf('/') + 1))
      },
      fetchWorkdirStatus: async (): Promise<WorkdirStatusList> => {
        return service.fetchWorkdirStatus()
      },
      settingsView: Settings,
    },
  })

});