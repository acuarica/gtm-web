import { GitService } from '@gtm/gtm'
import App from '../app/App.svelte'
import Settings from './Settings.svelte'
import { Commit, WorkdirStatusList } from '@gtm/notes';
import * as path from 'path';
import { spawn } from 'child_process';

const gtmservpath = path.join(__dirname, 'gtmserv')
console.info('Path to gtmserv:', gtmservpath)

window.addEventListener('DOMContentLoaded', async () => {

  const service = new GitService(args => spawn(gtmservpath, args))
  console.info('Starting gtm Dashboard using gtm service:', await service.getVersion())

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