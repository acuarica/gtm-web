
import { fetchCommits, fetchProjectList, fetchWorkdirStatus } from '../git.js'

import '../../main.pcss'

import App from '../app/App.svelte'
import Settings from './Settings.svelte'

window.addEventListener("DOMContentLoaded", async () => {
  const replaceText = (selector, text) => {
    const element = document.getElementById(selector);
    if (element) {
      element.innerText = text;
    }
  };

  console.info("Creating app with gtm/git service")
  console.log("@preload", document.body)
  new App({
    target: document.body,
    props: {
      fetchCommits: async (range) => {
        return fetchCommits(range)
      },
      fetchProjectList: async () => {
        return (await fetchProjectList()).map(p => p.substring(p.lastIndexOf("/") + 1))
      },
      fetchWorkdirStatus: async () => {
        return fetchWorkdirStatus()
      },
      settingsView: Settings,
    }
  })

  // const s = g.fetchCommits()
  // console.log(s, '@preload')
  // const m = await s
  // console.log(m, '@preload')

  for (const type of ["chrome", "node", "electron"]) {
    replaceText(`${type}-version`, (process.versions)[type]);
  }
});