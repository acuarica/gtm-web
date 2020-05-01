
import assert from 'assert';
import { commits } from '@gtm/mock';
import { computeStats } from '@gtm/notes';

describe('compute', () => {

  describe('computeStats', () => {

    it('pads 0', () => {
      const stats = computeStats(commits as any)

      let total = 0
      for (const p in stats.projects) {
        const project = stats.projects[p]

        console.log(project.name)
        total += project.total
      }

      assert.equal(stats.totalSecs, total)
    })
  })
})