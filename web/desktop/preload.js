import { GitService } from '@gtm/gtm';
import App from '../app/App.svelte';
import Settings from '../app/Settings.svelte';
import * as path from 'path';
import { spawn } from 'child_process';
const gtmservpath = path.join(__dirname, 'gtmserv');
console.info('Path to gtmserv:', gtmservpath);
window.addEventListener('DOMContentLoaded', async () => {
    const service = new GitService(args => spawn(gtmservpath, args));
    const version = await service.getVersion();
    console.info('Starting gtm Dashboard using gtm service:', version);
    new App({
        target: document.body,
        props: {
            fetchCommits: (filter) => service.fetchCommits(filter),
            fetchProjectList: async () => {
                const ps = await service.fetchProjectList();
                return ps.map(p => p.substring(p.lastIndexOf('/') + 1));
            },
            fetchWorkdirStatus: async () => {
                return service.fetchWorkdirStatus();
            },
            settingsView: Settings,
            settingsViewProps: (() => {
                const versions = {
                    'gtm Service': version,
                };
                const selectedComponents = {
                    node: 'Node.js',
                    chrome: 'Chromium',
                    electron: 'Electron',
                    v8: 'v8',
                };
                for (const key in selectedComponents) {
                    versions[selectedComponents[key]] =
                        process.versions[key];
                }
                return { versions: versions };
            })()
        },
    });
});
