import { GitService } from '../../@git/src/git.js'
import App from '../app/App.svelte'
import Settings from './Settings.svelte'
import { Commit, WorkdirStatusList } from '@gtm/notes';
import * as path from 'path';
import { spawn } from 'child_process';

const gtmpath = path.join(__dirname, 'gtm')
console.info(`Using gtm path: '${gtmpath}'`)

window.addEventListener('DOMContentLoaded', async () => {

  // const service = new GitService((args: string[]) => spawn('gtm', args))
  const service = new GitService(args => spawn(gtmpath, args))

  console.info('Running gtm app using gtm Dashboard service')
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