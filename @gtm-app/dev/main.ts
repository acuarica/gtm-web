import '@fortawesome/fontawesome-free/css/all.css'

import App from '../src/App.svelte'

import { commits } from '@gtm/mock'
import { projects } from '@gtm/mock'
import { workdir } from '@gtm/mock'

// let commitsDataUrl: string
// if (true || process.env.NODE_ENV === 'development') {
//   commitsDataUrl = '/data/commits'
//   commitsDataUrl = 'http://localhost:8080/data/commits'
// } else {
//   commitsDataUrl = '/gtm-web/data-commits.json'
// }

function delay<T>(func: () => T, timeout: number): Promise<T> {
  return new Promise(function (resolve, ) {
    setTimeout(() => {
      resolve(func())
    }, timeout);
  })
}

export function delayError<T>(func: () => T, timeout: number): Promise<T> {
  return new Promise(function (_, reject) {
    setTimeout(() => {
      reject(func())
    }, timeout);
  })
}

new App({
  target: document.body,
  props: true ?
    {
      fetchCommits: async (): Promise<typeof commits> => {
        return delay(() => {
          commits.push(commits[0])
          return commits
        }, 3000)
      },
      fetchProjectList: async (): Promise<string[]> => {
        return delay(() => {
          return projects.map(p => p.substring(p.lastIndexOf("/") + 1))
        }, 3000)
      },
      fetchWorkdirStatus: async (): Promise<typeof workdir> => {
        return delay(() => {
          return workdir
        }, 3000)
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
