import '@fortawesome/fontawesome-free/css/all.css'

import App from './app/App.svelte'

import commits from './data-commits.json'
import projects from './data-projects.json'
import workdir from './data-workdir.json'

new App({
  target: document.body,
  props: {
    fetchCommits: async (): Promise<typeof commits> => {
      return commits
    },
    fetchProjectList: async (): Promise<string[]> => {
      return projects.map(p => p.substring(p.lastIndexOf("/") + 1))
    },
    fetchWorkdirStatus: async (): Promise<typeof workdir> => {
      return workdir
    },
  }
});

// export async function fetchCommits2(range): any {
//   const commitsDataUrl = "/data/commits";
//   const url = `${commitsDataUrl}?all&from=${range.start}&to=${range.end}`;
//   const json = await fetch(url).then(r => r.json());
//   return json;
// }

// export async function fetchCommits(range): any {
//   // const url = `${commitsDataUrl}?all&from=${range.start}&to=${range.end}`;
//   return commits
// }

// async function fetchProjectList(): Promise<string[]> {
//   return ['proj1', 'proj2', 'proj3']
//   // const url = "/data/projects";
//   // const response = await fetch(url);
//   // const json = await response.json();
//   // return json.map(p => p.substring(p.lastIndexOf("/") + 1));
// }

// let commitsDataUrl: string
// if (true || process.env.NODE_ENV === 'development') {
//   commitsDataUrl = '/data/commits'
//   commitsDataUrl = 'http://localhost:8080/data/commits'
// } else {
//   commitsDataUrl = '/gtm-web/data-commits.json'
// }
