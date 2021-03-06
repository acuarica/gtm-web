import { GitService } from '@gtm/gtm'
import App from '../@app/App.svelte'
import Settings from '../@app/Settings.svelte'
import { Commit, WorkdirStatusList, CommitsFilter } from '@gtm/notes';
import * as path from 'path';
import { spawn } from 'child_process';

const gtmservpath = path.join(__dirname, 'gtmcli')
console.info('Path to gtmserv:', gtmservpath)

window.addEventListener('DOMContentLoaded', async () => {

  const service = new GitService(args => spawn(gtmservpath, args))
  const version = await service.getVersion()
  console.info('Starting gtm Dashboard using gtm service:', version)

  new App({
    target: document.body,
    props: {
      fetchCommits: (filter: CommitsFilter): Promise<Commit[]> => service.fetchCommits(filter),
      fetchProjectList: async (): Promise<string[]> => {
        const ps = await service.fetchProjectList()
        return ps.map(p => p.substring(p.lastIndexOf('/') + 1))
      },
      fetchWorkdirStatus: async (): Promise<WorkdirStatusList> => {
        return service.fetchWorkdirStatus()
      },

      settingsView: class extends Settings {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        constructor(opts: { target: Element; props?: Record<string, any> | undefined }) {
          const versions: { [id: string]: string | null } = {
            'gtm Service': version,
          }
          const selectedComponents: { [id: string]: string } = {
            node: 'Node.js',
            chrome: 'Chromium',
            electron: 'Electron',
            v8: 'v8',
          }
          for (const key in selectedComponents) {
            versions[selectedComponents[key]] =
              process.versions[key as keyof NodeJS.ProcessVersions]
          }
          super({ ...opts, props: { versions: versions } })
        }
      }

    },
  })

});