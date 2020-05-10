import { GitService } from '@gtm/git'
import { spawn } from 'child_process'
import assert from 'assert'

describe('@gtm/git', () => {

  const service = new GitService(
    (args: string[]) => spawn('node', ['@git/test/gtm-mock.js', ...args])
  )

  describe('fetchCommits', () => {

    it('throws with invalid range', async () => {
      const range = { start: '1asdf', end: '2asdf' }
      assert.throws(() => service.fetchCommits(range), 'rejects because of invalid date')
    })

    it('fetches empty commits', async () => {
      const range = { start: '2020-01-01', end: '2020-02-01' }
      const commits = await service.fetchCommits(range)
      assert.equal(commits.length, 0)
    })

    it('fetches commits', async () => {
      const range = { start: '2020-04-01', end: '2020-05-01' }
      const commits = await service.fetchCommits(range)
      assert(commits.length > 0, 'no commits')
    })

  })

})