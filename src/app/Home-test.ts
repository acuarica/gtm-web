import Home from './Home.svelte'
import { computeStats, computeWorkdirStatus } from '@gtm/notes'
import { WebService } from './web'

const web = new WebService()

export default [{
  component: Home,
  name: 'w/data',
  props: {
    statsPromise: web.fetchCommits({ start: '2020-01-01', end: '2020-04-30' }).then(value => computeStats(value)),
    workdirStatsPromise: web.fetchWorkdirStatus().then(value => computeWorkdirStatus(value))
  }
}]