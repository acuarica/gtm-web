
import { fetchCommits, fetchProjectList, fetchWorkdirStatus } from '../git.js'

import '../../main.pcss'

import App from '../app/App.svelte'
import Settings from './Settings.svelte'

window.addEventListener("DOMContentLoaded", async () => {

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

});