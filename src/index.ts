import App from './App.svelte';

new App({
  target: document.body,
  props: {
  }
});

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
