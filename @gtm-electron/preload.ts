
import * as g from '@gtm/git'

window.addEventListener("DOMContentLoaded", async () => {
  const replaceText = (selector: string, text: string) => {
    const element = document.getElementById(selector);
    if (element) {
      element.innerText = text;
    }
  };

  const s = g.fetchCommits()
  console.log(s, '@preload')
  const m = await s
  console.log(m, '@preload')

  for (const type of ["chrome", "node", "electron"]) {
    replaceText(`${type}-version`, (process.versions as any)[type]);
  }
});