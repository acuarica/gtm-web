import Project from './Project.svelte'
import { computeStats, computeWorkdirStatus } from '@gtm/notes'
import { WebService } from './web'

const web = new WebService()

export default [{
  component: Project,
  name: 'Project valid data',
  props: {
    name: 'web',
    projectPromise: web.fetchCommits({ start: '2020-01-01', end: '2020-04-30' }).then(value => computeStats(value).projects['web']),
    workdirStatsPromise: web.fetchWorkdirStatus().then(value => computeWorkdirStatus(value).projects['web'])
  }
}, {
  component: Project,
  name: 'Project not defined',
  props: {
    name: 'work',
    projectPromise: undefined,

  }
}, {
  component: Project,
  name: 'Project delay loading',
  props: {
    name: 'web',
    projectPromise: web.fetchCommits({ start: '2020-01-01', end: '2020-04-30' }).then(value => computeStats(value).projects['web']),
  }
}]