import '../../main.pcss'
import App from '../app/App.svelte'
import { Commit } from '@gtm/notes';

// let commitsDataUrl: string
// if (true || process.env.NODE_ENV === 'development') {
//   commitsDataUrl = '/data/commits'
// } else {
//   commitsDataUrl = '/gtm-web/data-commits.json'
// }

export function delay<T>(func: any, timeout: number): Promise<T> {
  return new Promise(function (resolve) {
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

console.info('Creating app with mock service')

const fetchurl = async <T>(url: string): Promise<T> => await fetch(url).then(r => r.json())

new App({
  target: document.body,
  props: {
    fetchCommits: async (range: { start: string; end: string }): Promise<Commit[]> => {
      // return delay(() => {
        const commitsDataUrl = '/data/commits'
        const url = `${commitsDataUrl}?all&from=${range.start}&to=${range.end}`
        return fetchurl(url)
      // }, 10)
    },
    fetchProjectList: async (): Promise<string[]> => {
      // return delay(async () => {
        const url = '/data/projects';
        return (await fetchurl<string[]>(url)).map((p: string) => p.substring(p.lastIndexOf('/') + 1));
      // }, 3000)
    },
    fetchWorkdirStatus: async (): Promise<{ [p: string]: Commit }> => {
      // return delay(async () => {
        const url = '/data/status';
        return await fetchurl(url)
      // }, 3000)
    },
  },
});
