import App from './App.svelte'
import { WebService } from './web'

const web = new WebService()

export default [{
  component: App,
  name: 'App with web service',
  props: {
    fetchCommits: web.fetchCommits,
    fetchProjectList: web.fetchProjectList,
    fetchWorkdirStatus: web.fetchWorkdirStatus,
  }
}]