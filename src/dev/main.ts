import App from '../app/App.svelte'
import { Commit } from '@gtm/notes';

console.debug('Creating main app with web service')

const fetchurl = async <T>(url: string): Promise<T> => await fetch('http://localhost:8000'+url).then(r => r.json())

new App({
  target: document.body,
  props: {
    fetchCommits: async (range: { start: string; end: string }): Promise<Commit[]> => {
      const commitsDataUrl = '/data/commits'
      const url = `${commitsDataUrl}?all&from=${range.start}&to=${range.end}`
      return fetchurl(url)
    },
    fetchProjectList: async (): Promise<string[]> => {
      const url = '/data/projects';
      return (await fetchurl<string[]>(url)).map((p: string) => p.substring(p.lastIndexOf('/') + 1));
    },
    fetchWorkdirStatus: async (): Promise<{ [p: string]: Commit }> => {
      const url = '/data/status';
      return await fetchurl(url)
    },
  },
});
