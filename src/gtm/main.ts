import '../../main.pcss'
import App from '../app/App.svelte'
import { Commit } from '@gtm/notes';

console.debug('Creating main gtm app with rust service')

const fetchurl = async <T>(url: string): Promise<T> => await fetch('http://localhost:9090' + url).then(r => r.json())

const rpc = {
  invoke: function (arg: any) {
    (window.external as any).invoke(JSON.stringify(arg))
  },
  projects: function (): string[] {
    rpc.invoke({ cmd: 'addTask', name: name });
  },
}

new App({
  target: document.body,
  props: {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    fetchCommits: async (_range: { start: string; end: string }): Promise<Commit[]> => {
      return Promise.resolve([])
    },
    fetchProjectList: async (): Promise<string[]> => {
      const projects = rpc.projects()
      return projects.map((p: string) => p.substring(p.lastIndexOf('/') + 1));
    },
    fetchWorkdirStatus: async (): Promise<{ [p: string]: Commit }> => {
      return Promise.resolve({})
    },
  },
});
