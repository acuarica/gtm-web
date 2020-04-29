
// import '@fortawesome/free-solid-svg-icon'
import '@fortawesome/fontawesome-free/css/all.css'

import App from './src/App.svelte'

import commits from './src/data-commits.json'
import projects from './src/data-projects.json'
import workdir from './src/data-workdir.json'

new App({
  target: document.body,
  props: {
    fetchCommits: async (): Promise<typeof commits> => {
      const commitsDataUrl = "/data/commits";
      // const url = `${commitsDataUrl}?all&from=${range.start}&to=${range.end}`;
      const url = commitsDataUrl
      const json = await fetch(url).then(r => r.json());
      return json;
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

//     const commitCounts: number[] = []
//     const datasets = []
//     for (const pname in projects) {
//       const p = projects[pname]
//       commitCounts.push(p.commitcount)
//       datasets.push({
//         data: [p.total],
//         label: pname,
//       })
//     }
