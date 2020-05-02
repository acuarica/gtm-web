
import * as g from '@gtm/git'

import { App } from '@gtm/app'
import { commits, projects, workdir } from '@gtm/mock'

window.addEventListener("DOMContentLoaded", async () => {
  const replaceText = (selector: string, text: string) => {
    const element = document.getElementById(selector);
    if (element) {
      element.innerText = text;
    }
  };


  console.log("@preload", document.body)
  new App({
    target: document.body,
    props: {
      fetchCommits: async (range: any): Promise<typeof commits> => {
        return g.fetchCommits(range)
      },
      fetchProjectList: async (): Promise<string[]> => {
        return projects.map(p => p.substring(p.lastIndexOf("/") + 1))
      },
      fetchWorkdirStatus: async (): Promise<typeof workdir> => {
        return workdir
      }
    }
  })

  // const s = g.fetchCommits()
  // console.log(s, '@preload')
  // const m = await s
  // console.log(m, '@preload')

  for (const type of ["chrome", "node", "electron"]) {
    replaceText(`${type}-version`, (process.versions as any)[type]);
  }
});