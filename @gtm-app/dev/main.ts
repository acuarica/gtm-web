import '@fortawesome/fontawesome-free/css/all.css'

import App from '../src/App.svelte'

import commits from './data-commits.json'
import projects from './data-projects.json'
import workdir from './data-workdir.json'

// let commitsDataUrl: string
// if (true || process.env.NODE_ENV === 'development') {
//   commitsDataUrl = '/data/commits'
//   commitsDataUrl = 'http://localhost:8080/data/commits'
// } else {
//   commitsDataUrl = '/gtm-web/data-commits.json'
// }

new App({
  target: document.body,
  props: true ?
    {
      fetchCommits: async (): Promise<typeof commits> => {
        return commits
      },
      fetchProjectList: async (): Promise<string[]> => {
        return projects.map(p => p.substring(p.lastIndexOf("/") + 1))
      },
      fetchWorkdirStatus: async (): Promise<typeof workdir> => {
        return workdir
      }
    } : {
      fetchCommits: async (range: { start: string; end: string }): Promise<typeof commits> => {
        const commitsDataUrl = "/data/commits"
        const url = `${commitsDataUrl}?all&from=${range.start}&to=${range.end}`
        const json = await fetch(url).then(r => r.json())
        return json
      },
      fetchProjectList: async (): Promise<string[]> => {
        const url = "/data/projects";
        const response = await fetch(url);
        const json: string[] = await response.json();
        return json.map(p => p.substring(p.lastIndexOf("/") + 1));
      },
      fetchWorkdirStatus: async (): Promise<typeof workdir> => {
        return workdir
      },
    }
});

// export {App, commits, projects, workdir}