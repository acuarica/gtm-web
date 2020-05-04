import '@fortawesome/fontawesome-free/css/all.css'
import '../../main.pcss'

import App from '../app/App.svelte'

// let commitsDataUrl: string
// if (true || process.env.NODE_ENV === 'development') {
//   commitsDataUrl = '/data/commits'
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

const fetchurl = async (url) => await fetch(url).then(r => r.json())

new App({
  target: document.body,
  props: {
    fetchCommits: async (range) => {
      return delay(() => {
        const commitsDataUrl = "/data/data-commits.json"
        const url = `${commitsDataUrl}?all&from=${range.start}&to=${range.end}`
        return fetchurl(url)
      }, 10)
    },
    fetchProjectList: async () => {
      return delay(async () => {
        const url = "/data/data-projects.json";
        return (await fetchurl(url)).map(p => p.substring(p.lastIndexOf("/") + 1));
      }, 3000)
    },
    fetchWorkdirStatus: async () => {
      return delay(async () => {
        const url = "/data/data-workdir.json";
        return await fetchurl(url)
      }, 3000)
    }
  }
});
