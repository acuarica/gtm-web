import { fetchCommits } from '@gtm/git'

describe('@gtm/git', () => {

  describe('fetchCommits', () => {

    it('does not fail with invalid range', async () => {
      const range = { start: '1', end: '2' }
      await fetchCommits(range)
    })

    it('test git', async () => {
      const range = { start: '2020-04-01', end: '2020-04-30' }
      await fetchCommits(range)
      // console.log(c)
    })

  })

})