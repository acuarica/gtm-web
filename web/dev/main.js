import App from '../app/App.svelte';
import Settings from '../app/Settings.svelte';
import { WebService } from '../app/web';
export default (host) => {
    console.debug(`Creating main app with web service on ${host}`);
    const service = new WebService(host);
    new App({
        target: document.body,
        props: {
            fetchCommits: (filter) => service.fetchCommits(filter),
            fetchProjectList: () => service.fetchProjectList(),
            fetchWorkdirStatus: () => service.fetchWorkdirStatus(),
            settingsView: Settings,
            settingsViewProps: { versions: {} },
        },
    });
};
