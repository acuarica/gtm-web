import { hhmm, pad0 } from '../src/format'
import assert from 'assert';

describe('format', function () {

  describe('#pad0()', function () {

    it('asdf', () => {
      assert.equal(pad0(0), '00')
    })

    it('should return -1 when the value is not present', function () {
      assert.equal(pad0(123), '123')
    })

  })

})
