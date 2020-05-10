import fs from 'fs'
import assert from 'assert';
import { computeStats, getDaily } from '@gtm/notes';
import { hhmm } from '@gtm/notes';

const [commits, ,] = ['commits', 'projects', 'workdir']
  .map(name => {
    const bytes = fs.readFileSync(`assets/data/${name}.json`);
    return JSON.parse(bytes.toString());
  })

describe('compute', () => {

  describe('computeStats', () => {

    it('Checks empty commits stats has 0 total time', () => {
      const stats = computeStats([])
      assert.equal(stats.totalSecs, 0)
    })

    it('Checks total by project', () => {
      const stats = computeStats(commits)
      const check = (p: string, expected: string): void => assert.equal(hhmm(stats.projects[p].total), expected)
      check('web', '79h 22m')
    })

    it('Checks total time must be sum of time of projects', () => {
      const stats = computeStats(commits)

      let total = 0
      for (const p in stats.projects) {
        const project = stats.projects[p]
        total += project.total
      }

      assert.equal(stats.totalSecs, total)
    })
  })

  it('Checks daily of empty projects is empty', () => {
    const daily = getDaily({})
    assert.equal(Object.keys(daily), 0)
  })

})