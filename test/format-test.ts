import { hhmm, pad0 } from '../src/gtm'
import assert from 'assert';

describe('format', function () {

  describe('#pad0()', function () {

    it('asdf', () => {
      assert.equal(pad0(0), '00')
    })

    it('two digits', () => {
      assert.equal(pad0(1), '01')
    })

    it('two digits', () => {
      assert.equal(pad0(19), '19')
    })

    it('should return -1 when the value is not present', function () {
      assert.equal(pad0(123), '123')
    })

  })

  describe('#hhmm()', function () {

    it('hhmm', () => {
      assert.equal(hhmm(12 * 60 * 60 + 5 * 60), '12h 05m')
    })

    it('fetch', () => {
      assert.equal(hhmm(12 * 60 * 60 + 5 * 60), '12h 05m')

      const commitsDataUrl = "/data/projects";
      fetch(commitsDataUrl).then(r => r.json()).then(ps => {
        console.log(ps)
      });
    })
  })

})
