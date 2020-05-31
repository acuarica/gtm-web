import App from '../app/App.svelte';
import Settings from '../app/Settings.svelte';
import { MockService } from '@gtm/mock';
async function fetchUrl(name) {
    return await fetch(`data/${name}.json`).then(r => r.json());
}
console.debug('Creating app with demo service');
const service = MockService.create(fetchUrl);
new App({
    target: document.body,
    props: {
        fetchCommits: async (filter) => {
            return (await service).fetchCommits(filter);
        },
        fetchProjectList: async () => {
            return (await service).fetchProjectList();
        },
        fetchWorkdirStatus: async () => {
            return (await service).fetchWorkdirStatus();
        },
        settingsView: Settings,
        settingsViewProps: { versions: { 'gtm': 'demo' } },
    },
});
