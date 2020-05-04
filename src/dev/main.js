import '@fortawesome/fontawesome-free/css/all.css'
import '../../main.pcss'

import App from '../app/App.svelte'

import { commits } from '../mock'
import { projects } from '../mock'
import { workdir } from '../mock'

// let commitsDataUrl: string
// if (true || process.env.NODE_ENV === 'development') {
//   commitsDataUrl = '/data/commits'
//   commitsDataUrl = 'http://localhost:8080/data/commits'
// } else {
//   commitsDataUrl = '/gtm-web/data-commits.json'
// }

function delay(func, timeout) {
  return new Promise(function (resolve, ) {
    setTimeout(() => {
      resolve(func())
    }, timeout);
  })
}

export function delayError(func, timeout) {
  return new Promise(function (_, reject) {
    setTimeout(() => {
      reject(func())
    }, timeout);
  })
}

console.info("Creating app with mock service")

new App({
  target: document.body,
  props: false ?
    {
      fetchCommits: async () => {
        return delay(() => {
          commits.push(commits[0])
          return commits
        }, 3000)
      },
      fetchProjectList: async () => {
        return delay(() => {
          return projects.map(p => p.substring(p.lastIndexOf("/") + 1))
        }, 3000)
      },
      fetchWorkdirStatus: async () => {
        return delay(() => {
          return workdir
        }, 3000)
      }
    } : {
      fetchCommits: async (range) => {
        const commitsDataUrl = "/data/commits"
        const url = `${commitsDataUrl}?all&from=${range.start}&to=${range.end}`
        const json = await fetch(url).then(r => r.json())
        return json
      },
      fetchProjectList: async () => {
        const url = "/data/projects";
        const response = await fetch(url);
        const json = await response.json();
        return json.map(p => p.substring(p.lastIndexOf("/") + 1));
      },
      fetchWorkdirStatus: async () => {
        return workdir
      },
    }
});
