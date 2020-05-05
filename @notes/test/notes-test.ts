
import assert from 'assert';
import { commits } from '@gtm/mock';
import { computeStats, getDaily } from '@gtm/notes';

describe('compute', () => {

  describe('computeStats', () => {

    it('Checks empty commits stats has 0 total time', () => {
      const stats = computeStats([])
      assert.equal(stats.totalSecs, 0)
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