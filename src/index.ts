import App from './App.svelte'

import commits from './commits.json'
// import { Commit } from './gtm';


new App({
  target: document.body,
  props: {
    fetchCommits: async (): Promise<any[]> => commits,
    fetchProjectList: async (): Promise<string[]> => ['proj1', 'proj2', 'proj3'],
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
